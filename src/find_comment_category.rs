use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommentStyle {
    pub line: String,                    
    pub multiline_start: Option<String>, 
    pub multiline_end: Option<String>,
    pub alt_multiline: Option<(String, String)>
}

pub struct FindTypeOfComment {
    map: HashMap<String, CommentStyle>,
}

impl FindTypeOfComment{
    pub fn new() -> Self {

        Self { map: HashMap::new() }
    }

    pub fn default_set_style(&mut self){
        let mut mp = HashMap::new();

        mp.insert(
            "rs".to_string(),
            CommentStyle {
                line: "//".to_string(),
                multiline_start: Some("/*".to_string()),
                multiline_end: Some("*/".to_string()),
                alt_multiline: None
            },
        );

        mp.insert(
            "py".to_string(),
            CommentStyle {
                line: "#".to_string(),
                multiline_start: Some("'''".to_string()),
                multiline_end: Some("'''".to_string()),
                alt_multiline: Some(("\"\"\"".to_string(), "\"\"\"".to_string())),
            },
        );

        mp.insert(
            "cpp".to_string(),
            CommentStyle {
                line: "//".to_string(),
                multiline_start: Some("/*".to_string()),
                multiline_end: Some("*/".to_string()),
                alt_multiline: None
            },
        );

        mp.insert(
            "sh".to_string(),
            CommentStyle {
                line: "#".to_string(),
                multiline_start: None,
                multiline_end: None,
                alt_multiline: None
            },
        );

        mp.insert(
            "js".to_string(),
            CommentStyle {
                line: "//".to_string(),
                multiline_start: Some("/*".to_string()),
                multiline_end: Some("*/".to_string()),
                alt_multiline: None
            },
        );

        mp.insert(
            "java".to_string(),
            CommentStyle {
                line: "//".to_string(),
                multiline_start: Some("/*".to_string()),
                multiline_end: Some("*/".to_string()),
                alt_multiline: None
            },
        );

        mp.insert(
            "c".to_string(),
            CommentStyle {
                line: "//".to_string(),
                multiline_start: Some("/*".to_string()),
                multiline_end: Some("*/".to_string()),
                alt_multiline: None
            },
        );

        mp.insert(
            "yaml".to_string(),
            CommentStyle {
                line: "#".to_string(),
                multiline_start: None,
                multiline_end: None,
                alt_multiline: None
            },
        );

        self.map = mp;
    }

    pub fn extract_from_json(&mut self,src: PathBuf){
        let json_content = if let Ok(content) = fs::read_to_string(&src){
            content
        }else{
            self.default_set_style();
            eprint!("Failed to load json file");
            return;
        };

        let map: HashMap<String, CommentStyle> = if let Ok(mp) = serde_json::from_str(&json_content){
            mp
        }else{
            self.default_set_style();
            eprintln!("Failed to parse JSON file");
            return;
        };

        self.map = map;
    }

    pub fn get_comment_style(&self, ext: &str) -> Option<CommentStyle> {
        self.map.get(ext).cloned()
    }
}
