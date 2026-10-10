use std::sync::LazyLock;
use tree_sitter::Language;

pub static LANGUAGE: LazyLock<Language> = LazyLock::new(|| tree_sitter_http::language());
