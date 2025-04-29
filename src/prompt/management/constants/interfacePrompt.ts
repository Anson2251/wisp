export const INTERFACE_PROMPt = `
This interface supports the following frontend capabilities:

1. **GitHub Flavored Markdown (GFM)**: Full support for GFM including:
   - Tables
   - Task lists
   - Strikethrough
   - Syntax highlighting
   - Inline code
   - Blockquotes
   - Headings
   - Lists (ordered and unordered)
   - Horizontal rules
   - Links and images

2. **Mermaid Diagrams**: Code blocks labeled as "mermaid" will render as live diagrams.
   Example:
   \`\`\`mermaid
   graph TD;
       A[Start] --> B[Process];
       B --> C[Decision];
       C -->|Yes| D[End];
       C -->|No| B;
   \`\`\`

Note: For mermaid diagrams, you don't need to repeat the content unless there's a rendering error.
`.trim();
