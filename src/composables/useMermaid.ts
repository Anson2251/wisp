import { createMermaidRenderer, type RenderOptions } from "mermaid-isomorphic"
import { ref, computed } from "vue"
import { hashContent, getCachedDiagram, putCachedDiagram, clearDiagramCache } from "../libs/commands"

const renderer = createMermaidRenderer()
const memoryCache = ref(new Map<string, DiagramCacheEntry>())
const CACHE_SIZE_LIMIT = 20

function pruneMemoryCache() {
  if (memoryCache.value.size > CACHE_SIZE_LIMIT) {
    const keys = Array.from(memoryCache.value.keys())
    memoryCache.value.delete(keys[0])
  }
}

export function useMermaid() {
  const renderDiagram = async (diagram: string, options?: RenderOptions): Promise<DiagramCacheEntry | null> => {
    const cacheKey = await hashContent(JSON.stringify({ diagram, options }))

    // Check memory cache first
    if (memoryCache.value.has(cacheKey)) {
      console.log("Memory cache hit for diagram:", diagram)
      return memoryCache.value.get(cacheKey)!
    }

    // Check SQLite cache
    const cached = await getCachedDiagram(cacheKey)
    if (cached) {
      console.log("SQLite cache hit for diagram:", diagram)
      memoryCache.value.set(cacheKey, cached)
      return cached
    }

    console.log("Cache miss for diagram:", diagram)
    const results = await renderer([diagram], options || {})
    if (results.length > 0) {
      const result = results[0]
      if (!result) return Promise.reject("Returned result is null or undefined")
      if (result.status === "rejected") return Promise.reject(result.reason)

      const entry = {
        svg: result.value.svg,
        height: Math.round(result.value.height),
        width: Math.round(result.value.width)
      }

      // Update both caches
      memoryCache.value.set(cacheKey, entry)
      pruneMemoryCache()
      await putCachedDiagram(cacheKey, entry)

      return entry
    }
    return null
  }

  const clearCache = async () => {
    memoryCache.value.clear()
    await clearDiagramCache()
  }

  return {
    renderDiagram,
    clearCache,
    cacheSize: computed(() => memoryCache.value.size)
  }
}

interface DiagramCacheEntry {
  svg: string
  height: number
  width: number
}
