import { createMermaidRenderer, type RenderOptions } from "mermaid-isomorphic"
import { ref, computed } from "vue"
import { hashContent, getCachedDiagram, putCachedDiagram, clearDiagramCache } from "../libs/commands"


function addWhiteBackgroundToSvg(svgString: string): string {
    const parser = new DOMParser();
    const svgDoc = parser.parseFromString(svgString, 'image/svg+xml');
    const svgElement = svgDoc.querySelector('svg');

    if (!svgElement) {
        throw new Error('Invalid SVG: Could not find <svg> element');
    }

    svgElement.setAttribute('style', 'background-color: white;')
	svgElement.setAttribute('viewport-fill', 'white')

    const serializer = new XMLSerializer();
    return serializer.serializeToString(svgDoc);
}


const renderer = createMermaidRenderer()
const memoryCache = ref(new Map<string, DiagramCacheEntry>())
const CACHE_SIZE_LIMIT = 20
const CACHE_PRUNE_COEFFICIENT = 0.6

function pruneMemoryCache() {
	if (memoryCache.value.size > CACHE_SIZE_LIMIT) {
		console.info("[useMermaid] Memory cache size exceeded limit, pruning...")
		Array.from(memoryCache.value.keys())
			.slice(0, memoryCache.value.size - Math.floor(CACHE_SIZE_LIMIT * CACHE_PRUNE_COEFFICIENT))
			.forEach(key => memoryCache.value.delete(key))
	}
}

export function useMermaid() {
	const renderDiagram = async (diagram: string, options?: RenderOptions): Promise<DiagramCacheEntry | null> => {
		const cacheKey = await hashContent(JSON.stringify({ diagram, options }))
		const render = async () => {

			// Check memory cache first
			if (memoryCache.value.has(cacheKey)) {
				console.info("[useMermaid] Memory cache hit for diagram", { cacheKey })
				return memoryCache.value.get(cacheKey)!
			}

			// Check SQLite cache
			const cached = await getCachedDiagram(cacheKey)
			if (cached) {
				console.info("[useMermaid] SQLite cache hit for diagram", { cacheKey })
				memoryCache.value.set(cacheKey, cached)
				return cached
			}

			const results = await renderer([diagram], options || {})
			if (results.length > 0) {
				const result = results[0]
				if (!result) return Promise.reject("Returned result is null or undefined")
				if (result.status === "rejected") return Promise.reject(result.reason)

				const height = Math.round(result.value.height)
				const width = Math.round(result.value.width)

				const entry = {
					svg: addWhiteBackgroundToSvg(result.value.svg),
					height,
					width
				}

				// Update both caches
				pruneMemoryCache()
				memoryCache.value.set(cacheKey, entry)
				putCachedDiagram(cacheKey, entry)
					.then(() => console.info("[useMermaid] Diagram cached in SQLite successfully", { cacheKey }))
				console.info("[useMermaid] Cache miss, rendered diagram", { cacheKey })
				return entry
			}
			return null
		}

		const identifier = `[useMermaid] Render diagram (${cacheKey})`
		console.time(identifier)
		const result = await render()
		console.timeEnd(identifier)
		return result
	}

	const clearCache = async () => {
		memoryCache.value.clear()
		console.info("[useMermaid] Memory cache cleared")
		await clearDiagramCache()
	}

	return {
		renderDiagram,
		clearCache,
		cacheSize: computed(() => memoryCache.value.size)
	}
}

export type DiagramCacheEntry = {
	svg: string
	height: number
	width: number
}
