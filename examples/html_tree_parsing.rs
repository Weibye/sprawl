//! This example shows how to parse a single html string into tokens.

use std::fs;

use html5ever::{
    tendril::format_tendril,
    tokenizer::{BufferQueue, Tokenizer, TokenizerOpts}, driver, ParseOpts,
};
use taffy::html::{SimpleTokenPrinter, TaffyTreeSink};

fn main() {
    // Setup input
    let content = fs::read_to_string("./examples/test.html").unwrap();
    let mut queue = BufferQueue::new();
    queue.push_back(format_tendril!("{}", content));

    // Setup the sink and tokenizer
    let sink = TaffyTreeSink;

    driver::parse_fragment(sink, ParseOpts::default());
}
