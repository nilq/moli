mod moli;
use moli::syntax;

use syntax::{lexer, parser};

use lexer::{BlockTree, process_branch};
use parser::{Traveler, Parser};

fn test() {
    let test = r#"
.123
-123
1234

true
false

100 + 100 * 100 % .123 * .1 
    "#;

    let mut blocks = BlockTree::new(test, 0);
    let indents    = blocks.indents();

    let root = blocks.tree(&indents);
    let done = process_branch(&root);

    let traveler = Traveler::new(done);
    let mut parser = Parser::new(traveler);

    for e in parser.parse() {
        println!("{:#?}", e)
    }
}

fn main() {
    test()
}