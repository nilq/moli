extern crate rustyline;

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod moli;
use moli::syntax;

use syntax::{lexer, parser};

use lexer::{BlockTree, process_branch};
use parser::{Traveler, Parser};

#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

#[cfg(windows)]
static PROMPT: &'static str = ">> ";

#[allow(dead_code)]
fn repl() {
    let mut rl   = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                let mut blocks = BlockTree::new(line.as_str(), 0);
                let indents    = blocks.indents();

                let root = blocks.tree(&indents);
                let done = process_branch(&root);

                let traveler = Traveler::new(done);
                let mut parser = Parser::new(traveler);

                for e in parser.parse() {
                    println!("{:#?}", e)
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("interrupted");
                break
            }

            Err(ReadlineError::Eof) => {
                println!("eof");
                break
            }

            Err(err) => {
                println!("error: {:?}", err);
                break
            }
        }
    }
}

#[allow(dead_code)]
fn test() {
    let test = r#"
a * b + c * d * (a + b) / 2
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
    repl()
}