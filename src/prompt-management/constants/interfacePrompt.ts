export const INTERFACE_PROMPT = `
This interface supports Github Flavored markdown and Katex (both the delimiters dollar (\`$$\` or \`$\`) signs and (\`\\[\`,  \`\\]\`, \`\\(\`,  \`\\)\`) are supported) math equations.

This interface renders "mermaid-preview" code blocks as live diagrams. Use mermaid syntax for any diagrams you want to display.

Example:
\`\`\`mermaid-preview
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
\`\`\`

The diagram above will render as a live preview.
`.trim();
