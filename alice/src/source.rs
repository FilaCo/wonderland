#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
    pub file_id: SourceFileId,
}

impl chumsky::span::Span for Span {
    type Context = SourceFileId;

    type Offset = u32;

    fn new(context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
            file_id: context,
        }
    }

    fn context(&self) -> Self::Context {
        self.file_id
    }

    fn start(&self) -> Self::Offset {
        self.lo
    }

    fn end(&self) -> Self::Offset {
        self.hi
    }
}

pub type Spanned<T> = (T, Span);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceFileId(u32);
