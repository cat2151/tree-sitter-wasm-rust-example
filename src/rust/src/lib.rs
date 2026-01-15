use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AstNode {
    #[serde(rename = "progression")]
    Progression { children: Vec<AstNode> },
    #[serde(rename = "note")]
    Note { text: String },
}

/// Convert note name to degree in C major scale
pub fn note_to_degree(note: &str) -> u8 {
    match note {
        "C" => 1,
        "D" => 2,
        "E" => 3,
        "F" => 4,
        "G" => 5,
        "A" => 6,
        "B" => 7,
        _ => 0,
    }
}

/// Process AST and convert notes to degrees
pub fn process_ast(ast: &AstNode) -> Vec<u8> {
    match ast {
        AstNode::Progression { children } => {
            children.iter()
                .filter_map(|child| match child {
                    AstNode::Note { text } => Some(note_to_degree(text)),
                    _ => None,
                })
                .collect()
        }
        AstNode::Note { text } => vec![note_to_degree(text)],
    }
}

#[wasm_bindgen]
pub fn process_chord_progression(ast_json: &str) -> String {
    match serde_json::from_str::<AstNode>(ast_json) {
        Ok(ast) => {
            let degrees = process_ast(&ast);
            serde_json::to_string(&degrees).unwrap_or_else(|_| "[]".to_string())
        }
        Err(e) => {
            format!("{{\"error\": \"{}\"}}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_degree() {
        assert_eq!(note_to_degree("C"), 1);
        assert_eq!(note_to_degree("D"), 2);
        assert_eq!(note_to_degree("E"), 3);
        assert_eq!(note_to_degree("F"), 4);
        assert_eq!(note_to_degree("G"), 5);
        assert_eq!(note_to_degree("A"), 6);
        assert_eq!(note_to_degree("B"), 7);
    }

    #[test]
    fn test_process_ast() {
        let ast = AstNode::Progression {
            children: vec![
                AstNode::Note { text: "C".to_string() },
                AstNode::Note { text: "F".to_string() },
                AstNode::Note { text: "G".to_string() },
                AstNode::Note { text: "C".to_string() },
            ],
        };
        assert_eq!(process_ast(&ast), vec![1, 4, 5, 1]);
    }

    #[test]
    fn test_process_chord_progression() {
        let json = r#"{"type":"progression","children":[{"type":"note","text":"C"},{"type":"note","text":"F"},{"type":"note","text":"G"},{"type":"note","text":"C"}]}"#;
        let result = process_chord_progression(json);
        assert_eq!(result, "[1,4,5,1]");
    }
}
