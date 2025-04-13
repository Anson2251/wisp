import { remark } from 'remark'
import html from 'remark-html'
import { rehype } from 'rehype'
import rehypeMermaid from 'rehype-mermaid'
import { provide, inject } from 'vue'

export function useMarkdown() {
	const markdownProcessor = {
		remark: remark().use(html),
		rehype: rehype().use(rehypeMermaid, {
			strategy: 'img-svg',
			errorFallback: () => { }
		})
	}

	const provideMarkdownProcessor = () => {
		provide('markdownProcessor', markdownProcessor)
	}

	const injectMarkdownProcessor = () => {
		return inject('markdownProcessor', markdownProcessor)
	}

	return {
		provideMarkdownProcessor,
		injectMarkdownProcessor
	}
}
