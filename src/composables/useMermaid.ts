import { ref, computed } from "vue"
import { hashContent, getCachedDiagram, putCachedDiagram, clearDiagramCache } from "../libs/commands"

import mermaid, { type MermaidConfig } from "mermaid";

/**
 * Renders Mermaid diagram code and extracts size and SVG.
 * Uses getBBox() as a fallback if attributes/viewBox are missing.
 * @param diagramCode - Mermaid diagram code string
 */
export async function renderMermaidWithSize(diagramCode: string, config: MermaidConfig): Promise<DiagramCacheEntry> {
	try {
		mermaid.initialize({ ...config, startOnLoad: false });

		const { svg } = await mermaid.render("temp-id", diagramCode);

		const parser = new DOMParser();
		const doc = parser.parseFromString(svg, "image/svg+xml");
		const svgEl = doc.documentElement;

		if (!svgEl) return Promise.reject("SVG element not found in rendered diagram");

		let width = parseFloat(svgEl.getAttribute("width") || "0");
		let height = parseFloat(svgEl.getAttribute("height") || "0");

		// Try getBBox as a fallback if width/height are not found
		if ((!width || !height) && svgEl.hasAttribute("viewBox")) {
			const tempContainer = document.createElement("div");
			tempContainer.style.cssText = "position:absolute;visibility:hidden;";
			document.body.appendChild(tempContainer);

			const tempSvg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
			tempSvg.innerHTML = svgEl.innerHTML;
			tempContainer.appendChild(tempSvg);

			try {
				const bbox = tempSvg.getBBox();
				width = bbox.width;
				height = bbox.height;
			} catch (e) {
				console.warn("getBBox() failed:", e);
			}

			// Clean up
			document.body.removeChild(tempContainer);
		}

		return { width, height, svg };
	}
	catch (e) {
		return Promise.reject(`Error rendering Mermaid diagram: ${e}`);
	}
}

function addBackgroundToSvg(svgString: string, colour: string): string {
	const parser = new DOMParser();
	const svgDoc = parser.parseFromString(svgString, 'image/svg+xml');
	const svgElement = svgDoc.querySelector('svg');

	if (!svgElement) {
		throw new Error('Invalid SVG: Could not find <svg> element');
	}

	svgElement.setAttribute('style', `background-color: ${colour};`)
	svgElement.setAttribute('viewport-fill', colour)

	const serializer = new XMLSerializer();
	return serializer.serializeToString(svgDoc);
}

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

const getDiagram = async (diagram: string, options: MermaidConfig) => {
	const result = await renderMermaidWithSize(diagram, options)
	if (!result) return Promise.reject("Returned result is null or undefined")

	return {
		svg: result.svg,
		height: Math.round(result.height),
		width: Math.round(result.width)
	}
}

const renderWithCache = async (diagram: string, options: MermaidConfig) => {
	// Check memory cache first
	if (memoryCache.value.has(diagram)) {
		// console.info("[useMermaid] Memory cache hit for diagram", { cacheKey })
		return memoryCache.value.get(diagram)!
	}

	const cacheKey = await hashContent(JSON.stringify({ diagram, options }))

	// Check SQLite cache
	const cached = await getCachedDiagram(cacheKey)
	if (cached) {
		// console.info("[useMermaid] SQLite cache hit for diagram", { cacheKey })
		memoryCache.value.set(cacheKey, cached)
		return cached
	}

	try {
		const entry = await getDiagram(diagram, options)

		// Update both caches
		pruneMemoryCache()
		memoryCache.value.set(cacheKey, entry)
		putCachedDiagram(cacheKey, entry)
			.then(() => console.info("[useMermaid] Diagram cached in SQLite successfully", { cacheKey }))
		console.info("[useMermaid] Cache miss, rendered diagram", { cacheKey })
		return entry
	}
	catch (error) {
		return Promise.reject(error)
	}
}

export function useMermaid() {
	const renderDiagram = async (diagram: string, options?: Omit<MermaidConfig, 'theme'> & { theme: 'light' | 'dark' }, bgColour = 'white', allowCache = true): Promise<DiagramCacheEntry | null> => {
		const mermaidOptions: MermaidConfig = {
			...options,
			theme: options?.theme === 'dark' ? 'dark' : 'default',
		}

		try {
			if (allowCache) {
				const result = await renderWithCache(diagram, mermaidOptions)
				result.svg = addBackgroundToSvg(result.svg, bgColour)
				return result
			}
			else {
				console.info('[useMermaid] Caching is disabled, rendering diagram without cache')
				const result = await getDiagram(diagram, mermaidOptions)
				result.svg = addBackgroundToSvg(result.svg, bgColour)
				return result
			}
		}
		catch (error) {
			return Promise.reject(error)
		}
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
