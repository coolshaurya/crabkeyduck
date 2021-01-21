#![derive(Debug, Clone)]
struct ParsedDocument<'a> {
    title: Option<&'a str>,
    attributes: Attributes<'a>,
    elements: Vec<BlockElement<'a>>,
}

#![derive(Debug, Clone)]
struct BlockElement<'a> {
    attributes: Attributes<'a>,
    content: BlockContent,
}

#![derive(Debug, Clone)]
enum BlockContent {
    Paragraph { text: Vec<InlineElement> },
}

#![derive(Debug, Clone)]
enum InlineElement<'a> {
    Text(&'a str),
}

#![derive(Debug, Clone)]
struct Attributes<'a> {
    attributes: HashMap<&'a str, &'a str>,
}
