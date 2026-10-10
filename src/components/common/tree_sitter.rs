use tree_sitter::Language;
use std::sync::LazyLock;

pub static LANGUAGE: LazyLock<Language> = LazyLock::new(|| {
    tree_sitter_http::language()
});
