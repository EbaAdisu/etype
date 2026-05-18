use rand::seq::SliceRandom;

static PYTHON: &str = include_str!("../../assets/code/python.txt");
static BASH: &str = include_str!("../../assets/code/bash.txt");
static RUST_CODE: &str = include_str!("../../assets/code/rust.txt");

#[derive(Debug, Clone, PartialEq)]
pub enum CodeLang {
    Python,
    Bash,
    Rust,
}

impl CodeLang {
    pub fn label(&self) -> &'static str {
        match self {
            CodeLang::Python => "Python",
            CodeLang::Bash => "Bash",
            CodeLang::Rust => "Rust",
        }
    }

    pub fn random() -> Self {
        let choices = [CodeLang::Python, CodeLang::Bash, CodeLang::Rust];
        let mut rng = rand::thread_rng();
        choices
            .choose(&mut rng)
            .cloned()
            .unwrap_or(CodeLang::Python)
    }
}

pub fn load_snippets(lang: &CodeLang) -> Vec<Vec<String>> {
    let raw = match lang {
        CodeLang::Python => PYTHON,
        CodeLang::Bash => BASH,
        CodeLang::Rust => RUST_CODE,
    };
    raw.split("\n---\n")
        .map(|s| s.trim_matches('\n'))
        .filter(|s| !s.is_empty())
        .map(|s| s.lines().map(String::from).collect())
        .collect()
}

pub fn random_snippet(lang: &CodeLang) -> Vec<String> {
    let snippets = load_snippets(lang);
    let mut rng = rand::thread_rng();
    snippets
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| vec!["# empty".to_string()])
}
