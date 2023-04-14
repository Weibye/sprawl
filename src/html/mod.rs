use alloc::borrow::Cow;
use html5ever::{
    tokenizer::{
        Doctype,
        Token::{self, CharacterTokens, CommentToken, DoctypeToken, ParseError, TagToken},
        TokenSink, TokenSinkResult,
    },
    tree_builder::TreeSink, ExpandedName, tendril::StrTendril,
};

#[derive(Clone)]
pub struct DomNode;

pub struct SimpleTokenPrinter;


impl TokenSink for SimpleTokenPrinter {
    type Handle = DomNode;
    
    fn process_token(&mut self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            DoctypeToken(doctype) => println!("{}: Doctype: {:?}", line_number, doctype.name),
            TagToken(tag) => println!("{}: Tag: {:?}, {:?}", line_number, tag.name, tag.kind),
            CommentToken(comment) => println!("{}: Comment: {:?}", line_number, comment),
            CharacterTokens(character) => println!("{}: Character token: {}", line_number, character),
            Token::NullCharacterToken => println!("Null character"),
            Token::EOFToken => println!("{}: End of file", line_number),
            ParseError(error) => println!("{}: Parsing error: {}", line_number, error),
        }
        
        TokenSinkResult::Continue
    }
}

pub struct TaffyTreeSink;

impl TreeSink for TaffyTreeSink {
    type Handle = DomNode;

    type Output = TaffyTreeSink;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        todo!()
    }

    fn get_document(&mut self) -> Self::Handle {
        DomNode
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        todo!()
    }

    fn create_element(
        &mut self,
        name: html5ever::QualName,
        attrs: Vec<html5ever::Attribute>,
        flags: html5ever::tree_builder::ElementFlags,
    ) -> Self::Handle {
        todo!()
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        todo!()
    }

    fn create_pi(&mut self, target: html5ever::tendril::StrTendril, data: html5ever::tendril::StrTendril) -> Self::Handle {
        todo!()
    }

    fn append(&mut self, parent: &Self::Handle, child: html5ever::tree_builder::NodeOrText<Self::Handle>) {
        todo!()
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn append_doctype_to_document(
        &mut self,
        name: html5ever::tendril::StrTendril,
        public_id: html5ever::tendril::StrTendril,
        system_id: html5ever::tendril::StrTendril,
    ) {
        todo!()
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        todo!()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        todo!()
    }

    fn set_quirks_mode(&mut self, mode: html5ever::tree_builder::QuirksMode) {
        todo!()
    }

    fn append_before_sibling(&mut self, sibling: &Self::Handle, new_node: html5ever::tree_builder::NodeOrText<Self::Handle>) {
        todo!()
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
        todo!()
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        todo!()
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        todo!()
    }
}
