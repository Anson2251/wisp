import { unified } from 'unified'
import remarkParse from 'remark-parse'
import remarkRehype from 'remark-rehype'
import remarkStringify from 'remark-stringify'
import remarkMath from 'remark-math'
import rehypeStringify from 'rehype-stringify'
import rehypeSanitize from 'rehype-sanitize'
import rehypeKatex from 'rehype-katex'
import rehypeRaw from 'rehype-raw'
import { rehypeVue } from 'rehype-vue'
import rehypeMermaid from 'rehype-mermaid'
import remarkGfm from 'remark-gfm'
import rehypePrism from 'rehype-prism'

function createProcessor() {
	return unified()
		.use(remarkParse)
		.use(remarkMath, {
			singleDollarTextMath: true
		})
		.use(remarkRehype, {
			allowDangerousHtml: true,
		})
		.use(rehypeRaw)
		.use(rehypeSanitize)
		.use(rehypeKatex, {
			output: "mathml"
		})
		.use(rehypeMermaid)
		.use(rehypePrism)
		.use(rehypeStringify)
}

export function useStreamingMarkdownRenderer(onAppend: (html: string) => void) {
	const processor = createProcessor()
	const parseProcessor = unified().use(remarkParse)
	const stringifyProcessor = unified().use(remarkStringify)
	let buffer = ""

	const onNewlyAppend = async (appended: string) => {
		buffer += appended


		const tree = parseProcessor.parse(buffer)
		// console.log(tree)
		if (tree.children.length <= 1) return

		// console.log(buffer)


		const treeCloned = JSON.parse(JSON.stringify(tree)) as (ReturnType<typeof parseProcessor.parse>);

		treeCloned.children = treeCloned.children.slice(-1)
		buffer = stringifyProcessor.stringify(treeCloned);

		tree.children = tree.children.slice(0, -1)
		onAppend((await processor.process(stringifyProcessor.stringify(tree))).toString())
	}

	return (text: string) => {
		onNewlyAppend(text)
	}
}

export function useVueMdastRenderer() {
	const processor = unified()
	.use(remarkParse)
	.use(remarkGfm)
	.use(remarkMath, {
		singleDollarTextMath: true
	})
	.use(remarkRehype, {
		allowDangerousHtml: true,
	})
	.use(rehypeRaw)
	.use(rehypeSanitize)
	.use(rehypeKatex, {
		output: "mathml"
	})
	.use(rehypeMermaid)
	.use(rehypePrism)
	.use(rehypeVue)

	return async (text: string) => {
		return (await processor.process(text)).result
	}
}
