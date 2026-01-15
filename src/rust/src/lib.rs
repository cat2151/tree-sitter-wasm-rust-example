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
/// Returns None for invalid notes (not in C major scale)
pub fn note_to_degree(note: &str) -> Option<u8> {
    match note {
        "C" => Some(1),
        "D" => Some(2),
        "E" => Some(3),
        "F" => Some(4),
        "G" => Some(5),
        "A" => Some(6),
        "B" => Some(7),
        _ => None,
    }
}

/// Process AST and convert notes to degrees
/// Filters out invalid notes that are not in C major scale
pub fn process_ast(ast: &AstNode) -> Vec<u8> {
    match ast {
        AstNode::Progression { children } => {
            children.iter()
                .filter_map(|child| match child {
                    AstNode::Note { text } => note_to_degree(text),
                    _ => None,
                })
                .collect()
        }
        AstNode::Note { text } => {
            note_to_degree(text).into_iter().collect()
        }
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
            serde_json::json!({ "error": e.to_string() }).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_degree() {
        assert_eq!(note_to_degree("C"), Some(1));
        assert_eq!(note_to_degree("D"), Some(2));
        assert_eq!(note_to_degree("E"), Some(3));
        assert_eq!(note_to_degree("F"), Some(4));
        assert_eq!(note_to_degree("G"), Some(5));
        assert_eq!(note_to_degree("A"), Some(6));
        assert_eq!(note_to_degree("B"), Some(7));
        assert_eq!(note_to_degree("X"), None);
        assert_eq!(note_to_degree(""), None);
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
