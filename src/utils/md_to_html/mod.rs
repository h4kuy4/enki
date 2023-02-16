pub fn render(md_text: String) -> String {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    md_it_treesitter::add(parser);

    let ast = parser.parse(md_text.as_str());
    ast.render()
}
