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
import rehypeMathJaxSvg from 'rehype-mathjax/svg'
import { h } from 'vue'
import { NP, NEquation } from 'naive-ui'
import { toVNode } from '../libs/to-vnode'
import { InlineCode } from 'mdast'

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
		// .use(rehypeKatex, {
		// 	output: "mathml"
		// })
		.use(rehypeMathJaxSvg)
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

export function useVNodeRenderer() {
	let processor = unified()
		.use(remarkParse)
		.use(remarkMath, {
			singleDollarTextMath: true
		})
		.use(remarkGfm)


	// if (mermaid) {
	// 	processor = processor.use(rehypeMermaid, {
	// 		strategy: mermaid ? 'inline-svg' : 'pre-mermaid'
	// 	})
	// }

	const getInlineMathComponent = (node: InlineCode) => h(NEquation, {
		katexOptions: {
			displayMode: false,
			output: 'mathml'
		},
		value: node.value,
		style: "font-size: 1.15em; padding: 2px"
	})

	const getMathComponent = (node: InlineCode) => h(NEquation, {
		katexOptions: {
			displayMode: true,
			output: 'mathml'
		},
		value: node.value,
		style: "font-size: 1.2em; padding: 4px"
	})


	return async (text: string) => {
		try {
			const tree = processor.parse(text)
			const vnode = toVNode(tree, {
				components: {
					inlineMath: getInlineMathComponent,
					math: getMathComponent
				}
			})
			return vnode
		}
		catch (error) {
			console.error(error)
			return null
		}
	}
}
