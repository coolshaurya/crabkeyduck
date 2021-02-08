use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct Document<'a> {
    attributes: BTreeMap<&'a str, &'a str>,
    blocks: Vec<Block<'a>>,
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    attributes: BTreeMap<&'a str, &'a str>,
    type_: BlockType,
    content: BlockContent,
}

enum BlockContent<'a> {
    Compound(Vec<Block<'a>>), 
    Simple(Vec<SimpleText<'a>>),
    Raw(Vec<RawText<'a>),
    Verbatim(Vec<VerbatimText<'a>>),
    Empty,
}

#[derive(Debug, Clone)]
enum BlockType {
    Paragraph,
    LiteralParagraph,
    Admonition,
    Comment,
    Example,
    Fenced,
    Listing,
    Literal,
    Open,
    Passthrough,
    Quote,
    Sidebar,
    Source,
    Stem,
    Table,
    Verse,
}

pub fn parse(string: &str) -> Result<Document> {
    let mut document = Document::default();
    Ok(document)
}

pub type Result<T> = std::result::Result<T, crate::Error>;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid")]
    InvalidDocument,
}
