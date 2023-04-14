//! This example shows how to parse a single html string into tokens.

use html5ever::{
    tendril::format_tendril,
    tokenizer::{BufferQueue, Tokenizer, TokenizerOpts},
};
use taffy::html::SimpleTokenPrinter;

fn main() {
    // Setup input
    let input = "<html><head><title>Parsing a HTML String</title></head><body><h1>Hello World</h1><p>Lorem ipsum dolor sit amet.</p></body></html>";
    let mut queue = BufferQueue::new();
    queue.push_back(format_tendril!("{}", input));

    // Setup the sink and tokenizer
    let sink = SimpleTokenPrinter;
    let mut tokenizer = Tokenizer::new(sink, TokenizerOpts::default());

    // Run the parsing
    let _ = tokenizer.feed(&mut queue);
    tokenizer.end();
}
