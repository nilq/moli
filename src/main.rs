mod moli;
use moli::syntax;
use syntax::lexer;

use lexer::{BlockTree, process_branch};

fn main() {
    let test = r#"
str foo = r"some string"
i08 boo = 8

bar = .1234
baz = -1234

~ a comment here
if bar == baz
  hrm = r'hey'
    "#;

    let mut blocks = BlockTree::new(test, 0);
    let indents    = blocks.indents();

    let root = blocks.tree(&indents);
    let done = process_branch(&root);

    println!("lexed =>\n{:#?}", done)
}
