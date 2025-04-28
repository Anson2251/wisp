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
`.trim();
