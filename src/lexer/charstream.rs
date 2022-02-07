use core::iter::Iterator;

pub trait CharStream {
    fn advance(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
}

pub struct StaticCharStream {
    buffer: Vec<char>,
    curr: usize,
}

impl StaticCharStream {
    pub fn new(input: &str) -> StaticCharStream {
        StaticCharStream{
            buffer: input.chars().collect(),
            curr: 0
        }
    }
}

impl CharStream for StaticCharStream {
    fn advance(&mut self) -> Option<char> {
        if self.curr >= self.buffer.len() {
            None
        } else {
            let c = self.buffer[self.curr];
            self.curr += 1;
            Some(c)
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.curr >= self.buffer.len() {
            None
        } else {
            Some(self.buffer[self.curr])
        }
    }
}

impl core::iter::Iterator for dyn CharStream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
