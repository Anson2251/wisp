import { unified, Plugin, Processor } from 'unified'
import remarkParse from 'remark-parse'
import remarkRehype from 'remark-rehype'
import remarkStringify from 'remark-stringify'
import remarkMath from 'remark-math'
import rehypeStringify from 'rehype-stringify'
import rehypeSanitize from 'rehype-sanitize'
import rehypeKatex from 'rehype-katex'
import rehypeRaw from 'rehype-raw'
import rehypeMermaid from 'rehype-mermaid'
import { MermaidRenderer } from 'mermaid-isomorphic'
import {findAndReplace} from 'mdast-util-find-and-replace'
import {visit} from 'unist-util-visit'
import { provide, inject } from 'vue'
import { toVNode } from "mdast-util-to-vnode"

import { Root } from 'mdast'


// async function processMermaid(diagram: string): Promise<string> {
//   const hash = await invoke<string>('hash_content', { content: diagram })
//   let cached = await invoke<string>('get_cached_render', { hash })

//   if (!cached) {
//     const processed = await unified()
//       .use(rehypeMermaid as any, {
//         strategy: 'img-svg',
//         errorFallback: () => ''
//       })
//       .process(toVFile(`\`\`\`mermaid\n${diagram}\n\`\`\``))

//     cached = processed.toString()
//     await invoke<void>('put_cached_render', {
//       hash,
//       astJson: diagram,
//       renderedHtml: cached
//     })
//   }

//   return cached
// }

const mathLatex2Gfm = (text: string) => {
	return text
	.replace(/\\\[([\s\S]+?)\\\]/g, (match: string) => `$$$$${match.slice(2, -2)}$$$$`)
	.replace(/\\\(([\s\S]+?)\\\)/g, (match: string) => `$$${match.slice(2, -2)}$$`)
}

// function remarkMathLatex2Gfm () {
// 	const self = (this as Processor<Root>)
// 	findAndReplace(self, [
// 		// Handle inline math \(...\)
// 		[/\\\(([\s\S]+?)\\\)/g, (match: string) => `$$${match.slice(2, -2)}$$`],
// 		// Handle block math \[...\]
// 		[/\\\[([\s\S]+?)\\\]/g, (match: string) => `$$$$${match.slice(2, -2)}$$$$`]
// 	])



// }

function createProcessor() {
	return unified()
		.use(remarkParse)
		.use(remarkMath, {
			singleDollarTextMath: true
		})
		.use(remarkRehype, {
			allowDangerousHtml: true,
		})

		// .use(rehypeMermaid, {
		// 	errorFallback: (_, diagram) => ({
		// 		type: "element",
		// 		tagName: "div",
		// 		properties: {},
		// 		data: {},
		// 		children: [
		// 			{
		// 				type: "text",
		// 				value: "Error rendering Mermaid diagram"
		// 			}
		// 		]
		// 	})
		// })
		.use(rehypeSanitize)
		.use(rehypeKatex, {
			output: "mathml"
		})
		.use(rehypeMermaid)
		.use(rehypeRaw)
		.use(rehypeStringify)

}

export function useMarkdown() {
	const processor = createProcessor()

	async function processMarkdown(text: string): Promise<string> {
		text = mathLatex2Gfm(text)
		const result = await processor.process(text)
		return result.toString()
	}

	const provideMarkdownProcessor = () => {
		provide('markdownProcessor', { processMarkdown })
	}

	const injectMarkdownProcessor = () => {
		return inject('markdownProcessor', { processMarkdown })
	}

	return {
		provideMarkdownProcessor,
		injectMarkdownProcessor,
		useStreamingMarkdownRenderer
	}
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
	const parseProcessor = unified()
		.use(remarkParse)
		// .use(remarkMathLatex2Gfm)
		.use(remarkMath)

	return async (text: string) => {
		const ast = parseProcessor.parse(text)
		// const processedAst = await parseProcessor.run(ast)
		// console.log(processedAst)
		return toVNode(ast, {
			components: {

			}
		})
	}
}
