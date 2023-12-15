use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

use rowan::NodeOrToken; // TODO: this can be accessed from a higher level
use oq3_lexer::{tokenize, Token};
use oq3_parser::SyntaxKind;
use oq3_syntax;
use oq3_syntax::{parse_text, SourceFile, GreenNode};
use semantic_ast::syntax_to_semantics;

// use source_file::source_file;
//use source_file::{SourceTrait};

#[derive(Parser)]
#[command(name = "demotest")]
#[command(about = "Demo of parser that parses and prints tokens or trees to stdout.")]
#[command(long_about = "
Demo of parser that parses and prints tokens or trees to stdout.

Commands are `lex`, `parse-green`, `parse`, and `semantic`.
`lex` prints a stream of tokens. `parse-green` prints the green tree. `parse` prints the (syntactic) red tree.
`semantic` prints the semantic AST, that is an AST that includes types and resolved symbols.
")]
struct Cli {
    #[command(subcommand)]
    /// This is the Cli command doc
    command: Option<Commands>,
}

// `value_name` expects bare word, not flag.
#[derive(Subcommand)]
enum Commands {

    /// Parse file to semantic ast
    SemanticString {
        #[arg(value_name = "FILENAME")]
        /// file name to read
        file_name: PathBuf,
    },


    /// Parse file to semantic ast
    Semantic {
        #[arg(value_name = "FILENAME")]
        /// file name to read
        file_name: PathBuf,
    },

    // /// Parse file to semantic ast, with verbose printing
    // SemanticDbg {
    //     #[arg(value_name = "FILENAME")]
    //     /// file name to read
    //     file_name: PathBuf,
    // },

    /// Parse file to `SyntaxNode`
    Parse {
        #[arg(value_name = "FILENAME")]
        /// file name to read
        file_name: PathBuf,
    },

    /// Parse file to `GreenNode`
    ParseGreen {
        #[arg(value_name = "FILENAME")]
        file_name: PathBuf,
    },

    /// Lex file to `Token`s
    Lex {
        #[arg(value_name = "FILENAME")]
        file_name: PathBuf,
    },

}

fn main() {
    use oq3_syntax::ast::HasModuleItem;
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {

        Some(Commands::SemanticString { file_name }) => {
            let source = read_example_source(&file_name);
            let file_name = Some("giraffe");
            let result = syntax_to_semantics::parse_source_string(source, file_name);
            result.print_errors();
            // for err in context.errors().iter() {
            //     println!("{}", err);
            // }
            result.program().print_ast_debug();
        }


        Some(Commands::Semantic { file_name }) => {
            let result = syntax_to_semantics::parse_source_file(&file_name);
            println!("Found errors: {}", result.any_errors());
            // result.print_errors();
            //result.program().print_ast_debug();
        }

        // Some(Commands::SemanticDbg { file_name }) => {
        //     let context = syntax_to_semantics::string_to_semantic(&read_example_source(file_name));
        //     println!("ERRORS {:?}", context.errors());
        //     context.program().print_ast_debug_pretty();
        // }

        Some(Commands::Parse { file_name }) => {
            let parsed_source = SourceFile::parse(&read_example_source(file_name));
            let parse_tree: SourceFile = parsed_source.tree();
            println!("Found {} items", parse_tree.items().collect::<Vec<_>>().len());
            // for item in parse_tree.items() {
            //     println!("*************** item: {:?}", item);
            // }
            let syntax_errors = parsed_source.errors();
            println!("Found {} parse errors:\n{:?}\n", syntax_errors.len(), syntax_errors);
           //  print_tree(parse_tree);
        }

        Some(Commands::ParseGreen { file_name }) => {
            let (green_node, syntax_errors) = parse_text(&read_example_source(file_name));
            println!("{:?}", green_node);
            println!("{:?}", green_node.kind());
            print_node_or_token(green_node, 0);
            println!("\nFound {} parse errors:\n{:?}", syntax_errors.len(), syntax_errors);
        }

        Some(Commands::Lex { file_name }) => {
            let tokens: Vec<Token> = tokenize(&read_example_source(file_name)).collect();
            for tok in tokens {
                println!("{:?}", tok);
            }
        }

        None => { // TODO should print usage here.
            println!("Commands are semantic, parse, parse-green, and lex")
        }
    }
}

fn read_example_source(file_path: &PathBuf) -> String {
    let contents = fs::read_to_string(file_path.clone())
        .expect(format!("Unable to read file {:?}", file_path).as_str());
    return contents
}

fn print_tree(file: SourceFile) {
    use oq3_syntax::ast::{AstNode};
    for item in file.syntax().descendants() {
        println!("{:?}: {:}", item, item);
    }
}

fn print_node_or_token(item: GreenNode, depth: usize) {
    let spcs = " ".repeat(depth);
    for (_i, child) in item.children().enumerate() {
//        println!("{}{}: {} : {:?}", spcs, i, child, child);
        match child {
            NodeOrToken::Node(node) => {
                print_node_or_token(node.to_owned(), depth + 1);
            },
            NodeOrToken::Token(token) => {
                let sk = SyntaxKind::from(token.kind().0);
                println!("{}  {:?} {:?}", spcs, sk, token.text());
            },
        };
    }
    println!("{}<", spcs);
}
