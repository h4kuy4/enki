use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::code::CodeBlock;
use markdown_it::plugins::cmark::block::fence::CodeFence;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct TreeSitterSnippet {
    pub html: String,
}

impl NodeValue for TreeSitterSnippet {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.text_raw(&self.html);
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<TreeSitterRule>();
}

#[derive(Debug)]
pub struct TreeSitterRule;
impl CoreRule for TreeSitterRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node, _| {
            if let Some(data) = node.cast::<CodeBlock>() {
                let content = Some(data.content.as_str());
                let html = render_html(content, String::from(""));
                node.replace(TreeSitterSnippet { html });
                return;
            }

            if let Some(data) = node.cast::<CodeFence>() {
                let language = data.info.clone();
                let content = Some(data.content.as_str());
                let html = render_html(content, language);
                node.replace(TreeSitterSnippet { html });
                return;
            }
        });
    }
}

fn language_to_emum(language: &String) -> Option<treelight::Language> {
    let language = language.trim();
    match language.to_lowercase().as_str() {
        "c" => Some(treelight::Language::C),
        "cpp" => Some(treelight::Language::Cpp),
        "csharp" => Some(treelight::Language::CSharp),
        "c-sharp" => Some(treelight::Language::CSharp),
        "rust" => Some(treelight::Language::Rust),
        "js" => Some(treelight::Language::Javascript),
        "javascript" => Some(treelight::Language::Javascript),
        "jsx" => Some(treelight::Language::JavascriptJsx),
        "javascriptjsx" => Some(treelight::Language::JavascriptJsx),
        "javascript-jsx" => Some(treelight::Language::JavascriptJsx),
        "ts" => Some(treelight::Language::Typescript),
        "typescript" => Some(treelight::Language::Typescript),
        "tsx" => Some(treelight::Language::TypescriptTsx),
        "typescripttsx" => Some(treelight::Language::TypescriptTsx),
        "typescript-tsx" => Some(treelight::Language::TypescriptTsx),
        "python" => Some(treelight::Language::Python),
        "java" => Some(treelight::Language::Java),
        "php" => Some(treelight::Language::Php),
        "go" => Some(treelight::Language::Go),
        "scala" => Some(treelight::Language::Scala),
        "haskell" => Some(treelight::Language::Haskell),
        "ruby" => Some(treelight::Language::Ruby),
        "julia" => Some(treelight::Language::Julia),
        "json" => Some(treelight::Language::Json),
        "bash" => Some(treelight::Language::Bash),
        _ => None,
    }
}

pub fn render_html(code: Option<&str>, language: String) -> String {
    let code = match code {
        Some("") => return "".to_string(),
        Some(code) => code,
        None => return "".to_string(),
    };

    let scope = match language_to_emum(&language) {
        Some(scope) => scope,
        None => {
            return format!(
                "<pre><code>{}</code></pre>",
                code.lines()
                    .map(|line| format!("<span class=\"line\">{}</span>", line))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    };

    let html: String = treelight::highlight_to_html(scope, code);

    let html = html
        .lines()
        .map(|line| format!("<span class=\"line\">{}</span>", line))
        .collect::<Vec<String>>()
        .join("\n");

    format!("<pre language=\"{}\"><code>{}</code></pre>", language, html)
}
