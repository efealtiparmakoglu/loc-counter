//! Language definitions: line/block comment syntaxes and file extensions.
//! The table is data — adding a language is adding a row, not code.

#[derive(Clone, Copy, Debug)]
pub struct LangDef {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    /// Line comment prefixes.
    pub line_comments: &'static [&'static str],
    /// Block comment (start, end) pairs; empty when the language has none.
    pub block_comments: &'static [(&'static str, &'static str)],
}

pub const LANGUAGES: &[LangDef] = &[
    LangDef { name: "Rust", extensions: &["rs"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "C", extensions: &["c", "h"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "C++", extensions: &["cpp", "cc", "cxx", "hpp"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Go", extensions: &["go"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Java", extensions: &["java"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "JavaScript", extensions: &["js", "mjs", "cjs"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "TypeScript", extensions: &["ts", "mts", "cts"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "TSX", extensions: &["tsx"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Swift", extensions: &["swift"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Kotlin", extensions: &["kt", "kts"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Python", extensions: &["py", "pyi"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "Shell", extensions: &["sh", "bash", "zsh"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "Ruby", extensions: &["rb"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "Perl", extensions: &["pl", "pm"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "YAML", extensions: &["yaml", "yml"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "TOML", extensions: &["toml"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "R", extensions: &["r"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "Makefile", extensions: &["mk"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "PHP", extensions: &["php"], line_comments: &["//", "#"], block_comments: &[("/*", "*/")] },
    LangDef { name: "C#", extensions: &["cs"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "HTML", extensions: &["html", "htm"], line_comments: &[], block_comments: &[("<!--", "-->")] },
    LangDef { name: "XML", extensions: &["xml", "svg"], line_comments: &[], block_comments: &[("<!--", "-->")] },
    LangDef { name: "Markdown", extensions: &["md", "markdown"], line_comments: &[], block_comments: &[("<!--", "-->")] },
    LangDef { name: "CSS", extensions: &["css"], line_comments: &[], block_comments: &[("/*", "*/")] },
    LangDef { name: "SCSS", extensions: &["scss"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "SQL", extensions: &["sql"], line_comments: &["--"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Lua", extensions: &["lua"], line_comments: &["--"], block_comments: &[("--[[", "]]")] },
    LangDef { name: "Haskell", extensions: &["hs"], line_comments: &["--"], block_comments: &[("{-", "-}")] },
    LangDef { name: "Elixir", extensions: &["ex", "exs"], line_comments: &["#"], block_comments: &[] },
    LangDef { name: "Zig", extensions: &["zig"], line_comments: &["//"], block_comments: &[] },
    LangDef { name: "Dart", extensions: &["dart"], line_comments: &["//"], block_comments: &[("/*", "*/")] },
    LangDef { name: "Vue", extensions: &["vue"], line_comments: &["//"], block_comments: &[("<!--", "-->")] },
    LangDef { name: "JSON", extensions: &["json"], line_comments: &[], block_comments: &[] },
];

pub fn by_extension(ext: &str) -> Option<&'static LangDef> {
    LANGUAGES.iter().find(|l| l.extensions.contains(&ext))
}
