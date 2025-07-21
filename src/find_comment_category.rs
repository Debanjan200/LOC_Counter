use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct CommentStyle<'a> {
    pub line: &'a str,                    // line comment symbol
    pub multiline_start: Option<&'a str>, // optional block start
    pub multiline_end: Option<&'a str>,   // optional block end
    pub alt_multiline: Option<(&'a str, &'a str)>
}

pub struct FindTypeOfComment<'a> {
    map: HashMap<&'a str, CommentStyle<'a>>,
}

impl<'a> FindTypeOfComment<'a> {
    pub fn new() -> Self {
        let mut mp = HashMap::new();

        mp.insert(
            "rs",
            CommentStyle {
                line: "//",
                multiline_start: Some("/*"),
                multiline_end: Some("*/"),
                alt_multiline: None
            },
        );

        mp.insert(
            "py",
            CommentStyle {
                line: "#",
                multiline_start: Some("'''"),
                multiline_end: Some("'''"),
                alt_multiline: Some(("\"\"\"", "\"\"\"")),
            },
        );

        mp.insert(
            "cpp",
            CommentStyle {
                line: "//",
                multiline_start: Some("/*"),
                multiline_end: Some("*/"),
                alt_multiline: None
            },
        );

        mp.insert(
            "sh",
            CommentStyle {
                line: "#",
                multiline_start: None,
                multiline_end: None,
                alt_multiline: None
            },
        );

        mp.insert(
            "js",
            CommentStyle {
                line: "//",
                multiline_start: Some("/*"),
                multiline_end: Some("*/"),
                alt_multiline: None
            },
        );

        mp.insert(
            "java",
            CommentStyle {
                line: "//",
                multiline_start: Some("/*"),
                multiline_end: Some("*/"),
                alt_multiline: None
            },
        );

        mp.insert(
            "c",
            CommentStyle {
                line: "//",
                multiline_start: Some("/*"),
                multiline_end: Some("*/"),
                alt_multiline: None
            },
        );

        mp.insert(
            "yaml",
            CommentStyle {
                line: "#",
                multiline_start: None,
                multiline_end: None,
                alt_multiline: None
            },
        );

        Self { map: mp }
    }

    pub fn get_comment_style(&self, ext: &str) -> Option<CommentStyle<'a>> {
        self.map.get(ext).cloned()
    }
}
