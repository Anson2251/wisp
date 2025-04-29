export const INTERFACE_PROMPT = `
Any code blocks labeled as "mermaid" will be rendered as live preview diagrams.
Please use the mermaid syntax for diagrams you'd like to be visualized.

Example:
\`\`\`mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
\`\`\`

The mermaid diagram above will be rendered as a live preview diagram by this interface.
You do not have to repeat the content of the diagram. But if user indicates the error of diagram rendering, you can repeat the content using a code block of text
`.trim();
