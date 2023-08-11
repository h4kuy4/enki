use markdown_it::common::utils::unescape_all;
use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::parser::inline::InlineRoot;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct CustomContainer {
    pub info: String,
    pub marker: char,
    pub marker_len: usize,
    pub container_prefix: &'static str,
}

impl NodeValue for CustomContainer {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let info = unescape_all(&self.info);
        let mut split = info.split_whitespace();
        let lang_name = split.next().unwrap_or("");
        let mut attrs = node.attrs.clone();
        let class;

        if !lang_name.is_empty() {
            class = format!("{}{}", self.container_prefix, lang_name);
            attrs.push(("class", class));
        }

        fmt.cr();
        fmt.open("div", &attrs);
        fmt.contents(&node.children);
        fmt.close("div");
        fmt.cr();
    }
}

#[derive(Debug, Clone, Copy)]
struct ContainerSettings(&'static str);
impl MarkdownItExt for ContainerSettings {}

impl Default for ContainerSettings {
    fn default() -> Self {
        Self("container-")
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<ContainerScanner>();
}

pub fn set_lang_prefix(md: &mut MarkdownIt, container_prefix: &'static str) {
    md.ext.insert(ContainerSettings(container_prefix));
}

#[doc(hidden)]
pub struct ContainerScanner;

impl ContainerScanner {
    fn get_header<'a>(state: &'a mut BlockState) -> Option<(char, usize, &'a str)> {
        if state.line_indent(state.line) >= 4 {
            return None;
        }

        let line = state.get_line(state.line);
        let mut chars = line.chars();

        let marker = chars.next()?;
        if marker != ':' {
            return None;
        }

        // scan marker length
        let mut len = 1;
        while Some(marker) == chars.next() {
            len += 1;
        }

        if len < 3 {
            return None;
        }

        let params = &line[len..];

        if marker == ':' && params.contains(marker) {
            return None;
        }

        Some((marker, len, params))
    }
}

impl BlockRule for ContainerScanner {
    fn check(state: &mut BlockState) -> Option<()> {
        Self::get_header(state).map(|_| ())
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let (marker, len, params) = Self::get_header(state)?;
        let params = params.to_owned();

        let mut next_line = state.line;
        let mut have_end_marker = false;

        // search end of block
        'outer: loop {
            next_line += 1;
            if next_line >= state.line_max {
                // unclosed block should be autoclosed by end of document.
                // also block seems to be autoclosed by end of parent
                break;
            }

            let line = state.get_line(next_line);

            if !line.is_empty() && state.line_indent(next_line) < 0 {
                // non-empty line with negative indent should stop the list:
                // - :::
                //  test
                break;
            }

            let mut chars = line.chars().peekable();

            if Some(marker) != chars.next() {
                continue;
            }

            if state.line_indent(next_line) >= 4 {
                continue;
            }

            // scan marker length
            let mut len_end = 1;
            while Some(&marker) == chars.peek() {
                chars.next();
                len_end += 1;
            }

            // closing code fence must be at least as long as the opening one
            if len_end < len {
                continue;
            }

            // make sure tail has spaces only
            loop {
                match chars.next() {
                    Some(' ' | '\t') => {}
                    Some(_) => continue 'outer,
                    None => {
                        have_end_marker = true;
                        break 'outer;
                    }
                }
            }
        }

        let indent = state.line_offsets[state.line].indent_nonspace;
        let (content, mapping) = state.get_lines(state.line + 1, next_line, indent as usize, true);

        let container_prefix = state
            .md
            .ext
            .get::<ContainerSettings>()
            .copied()
            .unwrap_or_default()
            .0;

        let mut node = Node::new(CustomContainer {
            info: params,
            marker,
            marker_len: len,
            container_prefix,
        });

        node.children
            .push(Node::new(InlineRoot::new(content, mapping)));

        Some((
            node,
            next_line - state.line + if have_end_marker { 1 } else { 0 },
        ))
    }
}
