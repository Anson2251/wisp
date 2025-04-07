//! Module for converting Markdown to Pango markup
//! 
//! Pango is the text rendering library used by GTK. This module converts
//! a subset of Markdown to Pango's markup format using recursive processing.

use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};

/// Escapes HTML special characters in text
pub fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&"),
            '<' => escaped.push_str("<"),
            '>' => escaped.push_str(">"),
            '"' => escaped.push_str("\""),
            '\'' => escaped.push_str("'"),
            _ => escaped.push(c),
        }
    }
    escaped
}

/// Converts markdown text to Pango markup
/// Returns Result with the converted text or an error message if invalid markup is detected
pub fn convert(text: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut tag_stack: Vec<&str> = Vec::new();
    let mut color_span_open = false;

    let parser = Parser::new(text);
    let mut events = parser.into_iter().peekable();

    while let Some(event) = events.next() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {}
                    Tag::Heading(level, _, _) => {
                        let size = match level {
                            HeadingLevel::H1 => "<big><big><big>",
                            HeadingLevel::H2 => "<big><big>",
                            _ => "<big>",
                        };
                        output.push_str(size);
                        tag_stack.push("heading");
                    }
                    Tag::BlockQuote => {
                        output.push_str("<i>");
                        tag_stack.push("blockquote");
                    }
                    Tag::CodeBlock(_) => {
                        output.push_str("<span foreground='#bbb' background='#222' font_family='monospace'>");
                        tag_stack.push("code");
                    }
                    Tag::List(Some(_)) => {
                        // Ordered list
                        if let Some(Event::Text(text)) = events.peek() {
                            output.push_str(&format!(" {} ", text));
                        }
                    }
                    Tag::List(None) => {
                        // Unordered list
                        output.push_str(" • ");
                    }
                    Tag::Item => {}
                    Tag::Emphasis => {
                        output.push_str("<i>");
                        tag_stack.push("emphasis");
                    }
                    Tag::Strong => {
                        output.push_str("<b>");
                        tag_stack.push("strong");
                    }
                    Tag::Strikethrough => {
                        output.push_str("<s>");
                        tag_stack.push("strikethrough");
                    }
                    Tag::Link(_, dest, _) => {
                        output.push_str(&format!("<a href='{}'>", dest));
                        tag_stack.push("link");
                    }
                    Tag::Image(_, dest, _) => {
                        output.push_str(&format!("<img src='{}'/>", dest));
                    }
                    _ => {} // Ignore unsupported tags
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Heading(level, _, _) => {
                        let size = match level {
                            HeadingLevel::H1 => "</big></big></big>",
                            HeadingLevel::H2 => "</big></big>",
                            _ => "</big>",
                        };
                        output.push_str(size);
                        tag_stack.pop();
                    }
                    Tag::BlockQuote => {
                        output.push_str("</i>");
                        tag_stack.pop();
                    }
                    Tag::CodeBlock(_) => {
                        output.push_str("</span>");
                        tag_stack.pop();
                    }
                    Tag::Emphasis => {
                        output.push_str("</i>");
                        tag_stack.pop();
                    }
                    Tag::Strong => {
                        output.push_str("</b>");
                        tag_stack.pop();
                    }
                    Tag::Strikethrough => {
                        output.push_str("</s>");
                        tag_stack.pop();
                    }
                    Tag::Link(_, _, _) => {
                        output.push_str("</a>");
                        tag_stack.pop();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                output.push_str(&escape_html(&text));
            }
            Event::Code(text) => {
                output.push_str(&format!("<span font_family='monospace'>{}</span>", escape_html(&text)));
            }
            Event::Html(html) => {
                // Handle custom HTML comments for colors
                if html.starts_with("<!--") && html.ends_with("-->") {
                    if html.contains("fg=") || html.contains("bg=") {
                        if color_span_open {
                            output.push_str("</span>");
                            color_span_open = false;
                        }
                        
                        let mut attrs = String::new();
                        if let Some(fg) = html.split("fg=").nth(1).and_then(|s| s.split_whitespace().next()) {
                            attrs.push_str(&format!(" foreground='{}'", fg.trim_matches(|c| c == ' ' || c == '\'' || c == '"')));
                        }
                        if let Some(bg) = html.split("bg=").nth(1).and_then(|s| s.split_whitespace().next()) {
                            attrs.push_str(&format!(" background='{}'", bg.trim_matches(|c| c == ' ' || c == '\'' || c == '"')));
                        }

                        if !attrs.is_empty() {
                            output.push_str(&format!("<span{}>", attrs));
                            color_span_open = true;
                        }
                    } else if html.contains("<!--/-->") && color_span_open {
                        output.push_str("</span>");
                        color_span_open = false;
                    }
                }
            }
            Event::SoftBreak => {
                output.push_str("");
            }
            Event::HardBreak => {
                output.push_str("\n");
            }
            Event::Rule => {
                output.push_str("\n\n");
            }
            _ => {} // Ignore unsupported events
        }
    }

    if color_span_open {
        output.push_str("</span>");
    }

    // Validate tag balance
    if !tag_stack.is_empty() {
        return Err(format!("Unclosed tags: {:?}", tag_stack));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let input = "# Heading\nSome *italic* and **bold** text";
        let expected = "<big><big><big>Heading</big></big></big>Some <i>italic</i> and <b>bold</b> text";
        assert_eq!(convert(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block() {
        let input = "```\ncode\n```";
        let expected = "<span foreground='#bbb' background='#222' font_family='monospace'>code</span>";
        assert_eq!(convert(input).unwrap().trim(), expected.trim());
    }

    #[test]
    fn test_nested_emphasis() {
        let input = "***bold and italic***";
        let expected = "<b><i>bold and italic</i></b>";
        assert_eq!(convert(input).unwrap(), expected);
    }

    #[test]
    fn test_nested_lists() {
        let input = "- Item 1\n  - Subitem 1\n  - Subitem 2\n- Item 2";
        let expected = " • Item 1 • Subitem 1 • Subitem 2 • Item 2";
        assert_eq!(convert(input).unwrap(), expected);
    }

    #[test]
    fn test_nested_blockquotes() {
        let input = "> Main quote\n> > Nested quote";
        let expected = "<i>Main quote</i><i>Nested quote</i>";
        assert_eq!(convert(input).unwrap(), expected);
    }

    #[test]
    fn test_unclosed_tag() {
        let input = "**bold but no closing";
        assert!(convert(input).is_err());
    }

    #[test]
    fn test_unclosed_span() {
        let input = "<span>text with no closing span";
        assert!(convert(input).is_err());
    }

    #[test]
    fn test_escaped_quotes() {
        let input = r#"text with "quotes" and 'apostrophes'"#;
        let expected = r#"text with "quotes" and 'apostrophes'"#;
        assert_eq!(convert(input).unwrap(), expected);
    }

    #[test]
    fn test_color_spans() {
        let input = "<!--fg=red-->colored text<!--/-->";
        let expected = "<span foreground='red'>colored text</span>";
        assert_eq!(convert(input).unwrap(), expected);
    }
}
