use std::str::Chars;

use memchr::memchr;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub(crate) fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub(crate) fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOI_CHAR)
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub(crate) fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && !self.is_at_eoi() {
            self.bump();
        }
    }

    pub(crate) fn eat_until(&mut self, byte: u8) {
        self.chars = match memchr(byte, self.as_str().as_bytes()) {
            Some(index) => self.as_str()[index..].chars(),
            None => "".chars(),
        }
    }

    /// Returns amount of already bumped symbols.
    pub(crate) fn bumped_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to 0.
    pub(crate) fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Checks if there is nothing more to consume.
    fn is_at_eoi(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

const EOI_CHAR: char = '\0';
