import { createMermaidRenderer, type RenderOptions } from "mermaid-isomorphic"
import { ref, computed } from "vue"

type CachedResult = {
	svg: string
	height: number
	width: number
}

const renderer = createMermaidRenderer()
const cache = ref(new Map<string, CachedResult>())

export function useMermaid() {
	const renderDiagram = async (diagram: string, options?: RenderOptions): Promise<CachedResult | null> => {
		const cacheKey = JSON.stringify({ diagram, options })

		if (cache.value.has(cacheKey)) {
			return cache.value.get(cacheKey)!
		}

		const results = await renderer([diagram], options || {})
		if (results.length > 0) {
			const result = results[0]
			if (!result) return Promise.reject("Returned result is null or undefined")

			if (result.status === "rejected") return Promise.reject(result.reason)
			const diagram = result.value

			const cached = {
				svg: diagram.svg,
				height: diagram.height,
				width: diagram.width
			}
			cache.value.set(cacheKey, cached)
			return cached
		}
		return null
	}

	const clearCache = () => {
		cache.value.clear()
	}

	return {
		renderDiagram,
		clearCache,
		cacheSize: computed(() => cache.value.size)
	}
}
