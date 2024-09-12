use crate::core::node::Traverser;
use crate::core::node::whitespace::Node;

#[test]
fn expect_token() {
    let mut traverser = Traverser::from("\n\n\t");
    dbg!(traverser.expect_tokens::<()>(&Node::WHITESPACE_TOKENS));
}