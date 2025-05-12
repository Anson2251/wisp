import { unified } from 'unified'
import remarkParse from 'remark-parse'
import remarkRehype from 'remark-rehype'
import remarkMath from 'remark-math'
import rehypeStringify from 'rehype-stringify'
import rehypeSanitize from 'rehype-sanitize'
import rehypeRaw from 'rehype-raw'
import remarkGfm from 'remark-gfm'
import rehypePrism from 'rehype-prism'
import { h } from 'vue'
import { NCode, NEquation } from 'naive-ui'
import CodeMermaidRenderer from '../components/CodeMermaidRenderer.vue'
import { toVNode } from '../libs/to-vnode'
import { Code, InlineCode, Root, RootContent } from 'mdast'


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


	return async (text: string, enableLastMermaid: boolean = false) => {
		try {
			const tree = processor.parse(text)

			const disableLastMermaidRc = (node: Root) => {
				if (!node) return
				if (!node.children) return;
				const last = node.children[node.children.length - 1]
				if (!last) return;

				if (last.type === 'code') {
					if (last.lang === 'mermaid') {
						last.lang = 'mermaid-generating'
					}
				}
				else {
					disableLastMermaidRc(node.children[node.children.length - 1] as unknown as Root)
				}
			}

			if (!enableLastMermaid) disableLastMermaidRc(tree)

			const vnode = toVNode(tree, {
				components: {
					inlineMath: getInlineMathComponent,
					math: getMathComponent,
					code: (node: Code) => h(CodeMermaidRenderer, { code: node.value ?? "", language: node.lang ?? "" }),
					inlineCode: (node: InlineCode) => h(NCode, { code: node.value ?? "", inline: true, wordWrap: true, style: {transition: 'none !important'} }),
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
