use chord_processor::{process_ast, AstNode};
use std::env;
use tree_sitter::{Parser, Node};

/// Convert Tree-sitter CST Node to JSON AST
fn node_to_json(node: Node, source: &str) -> serde_json::Value {
    let kind = node.kind();
    
    match kind {
        "progression" => {
            let mut children = Vec::new();
            let mut cursor = node.walk();
            
            for child in node.children(&mut cursor) {
                if child.kind() == "note" {
                    children.push(node_to_json(child, source));
                }
            }
            
            serde_json::json!({
                "type": "progression",
                "children": children
            })
        }
        "note" => {
            let text = match node.utf8_text(source.as_bytes()) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Warning: UTF-8 decoding error for 'note' node: {}", e);
                    ""
                }
            };
            serde_json::json!({
                "type": "note",
                "text": text
            })
        }
        _ => {
            serde_json::json!({})
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <chord-progression>", args[0]);
        eprintln!("Example: {} \"C-F-G-C\"", args[0]);
        std::process::exit(1);
    }
    
    let input = &args[1];
    
    // Parse with Tree-sitter
    let mut parser = Parser::new();
    let language = tree_sitter_chordprog::language();
    parser
        .set_language(language)
        .expect("Failed to set Tree-sitter language for chord progression parser");
    
    let tree = parser
        .parse(input, None)
        .expect("Failed to parse input chord progression with Tree-sitter");
    let root = tree.root_node();
    
    // Convert CST to JSON AST
    let ast_json = node_to_json(root, input);
    
    // Deserialize JSON AST to Rust AST
    let ast: AstNode = serde_json::from_value(ast_json)
        .expect("Failed to deserialize chord progression AST from JSON");
    
    // Process AST
    let degrees = process_ast(&ast);
    
    // Output result
    println!("{}", degrees.iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(","));
}
