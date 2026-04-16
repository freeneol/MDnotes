#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn word_count(&self) -> usize {
        self.blocks.iter().map(|b| b.word_count()).sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Block {
    Heading(u8, Vec<InlineSpan>),
    Paragraph(Vec<InlineSpan>),
    List(Vec<ListItem>),
    BlockQuote(Vec<Block>),
    CodeBlock(CodeBlock),
    Image(ImageBlock),
    Table(Table),
    HorizontalRule,
}

impl Block {
    fn word_count(&self) -> usize {
        match self {
            Block::Heading(_, spans) | Block::Paragraph(spans) => {
                spans.iter().map(|s| s.word_count()).sum()
            }
            Block::List(items) => items.iter().map(|item| {
                item.spans.iter().map(|s| s.word_count()).sum::<usize>()
            }).sum(),
            Block::BlockQuote(blocks) => blocks.iter().map(|b| b.word_count()).sum(),
            Block::CodeBlock(cb) => cb.content.split_whitespace().count(),
            Block::Table(table) => {
                table.header.iter().chain(table.rows.iter().flatten())
                    .map(|cell| cell.iter().map(|s| s.word_count()).sum::<usize>())
                    .sum()
            }
            Block::Image(_) | Block::HorizontalRule => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub header: Vec<Vec<InlineSpan>>,
    pub rows: Vec<Vec<Vec<InlineSpan>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub ordered: bool,
    pub spans: Vec<InlineSpan>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageBlock {
    pub url: String,
    pub alt: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineSpan {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub link: Option<String>,
}

#[allow(dead_code)]
impl InlineSpan {
    pub fn new_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            bold: false,
            italic: false,
            code: false,
            link: None,
        }
    }

    pub fn with_bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn with_italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn with_code(mut self) -> Self {
        self.code = true;
        self
    }

    pub fn with_link(mut self, url: &str) -> Self {
        self.link = Some(url.to_string());
        self
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}
