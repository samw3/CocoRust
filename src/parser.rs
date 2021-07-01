use crate::scanner::Scanner;
use crate::tab::Tab;
use crate::trace::Trace;

pub struct Errors {
    pub count: u32,
}

pub struct Parser {
    pub scanner: Scanner,
    pub trace: Trace,
    pub tab: Tab,
    pub errors: Errors,
}

impl Parser {
    pub fn new(
        scanner: Scanner,
        trace: Trace,
        tab: Tab,
    ) -> Parser {
        Parser {
            scanner,
            trace,
            tab,
            errors: Errors { count: 0 },
        }
    }

    pub fn parse(&self) {
        todo!()
    }
}
