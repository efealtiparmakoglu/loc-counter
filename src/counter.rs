//! The counting engine: per-line classification with block-comment and
//! string-literal state machines. Pure logic, no I/O.

use crate::langs::LangDef;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub files: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
}

impl Counts {
    pub fn total(&self) -> usize {
        self.code + self.comments + self.blanks
    }
    pub fn add(&mut self, other: &Counts) {
        self.files += other.files;
        self.code += other.code;
        self.comments += other.comments;
        self.blanks += other.blanks;
    }
}

#[derive(Debug)]
struct State {
    in_block: Option<&'static str>, // the block-close token we await
}

/// Counts one file's lines for the given language.
pub fn count_lines(content: &str, lang: &LangDef) -> Counts {
    let mut counts = Counts { files: 1, ..Default::default() };
    let mut state = State { in_block: None };

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() && state.in_block.is_none() {
            counts.blanks += 1;
            continue;
        }
        if classify_line(line, lang, &mut state) == LineKind::Code {
            counts.code += 1;
        } else {
            counts.comments += 1;
        }
    }
    counts
}

#[derive(PartialEq)]
enum LineKind {
    Code,
    Comment,
}

/// Classifies a single line, advancing the block-comment state machine.
fn classify_line(line: &str, lang: &LangDef, state: &mut State) -> LineKind {
    let mut rest = line;
    let mut saw_code = false;
    let mut started_in_block = state.in_block.is_some();

    loop {
        if let Some(closer) = state.in_block {
            // everything until the closer is comment
            match rest.find(closer) {
                Some(pos) => {
                    rest = &rest[pos + closer.len()..];
                    state.in_block = None;
                }
                None => return LineKind::Comment,
            }
        }

        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }

        // block comment opener?
        let mut block_hit = false;
        for (opener, closer) in lang.block_comments {
            if rest.starts_with(opener) {
                state.in_block = Some(closer);
                rest = &rest[opener.len()..];
                block_hit = true;
                break;
            }
        }
        if block_hit {
            continue;
        }

        // line comment?
        if lang.line_comments.iter().any(|p| rest.starts_with(p)) {
            if !saw_code {
                return LineKind::Comment;
            }
            // trailing comment after code: counts as code
            return LineKind::Code;
        }

        // otherwise: code; skip string literals so comment openers inside
        // strings ("http://x", "/* not */") don't fool us
        saw_code = true;
        match next_token_end(rest) {
            Some(consumed) => rest = &rest[consumed..],
            None => break,
        }
    }

    if saw_code || started_in_block && !rest.is_empty() && state.in_block.is_none() {
        LineKind::Code
    } else if started_in_block {
        LineKind::Comment
    } else {
        LineKind::Comment
    }
}

/// Advances past one string literal starting at rest[0] ('...' or "..."),
/// honoring backslash escapes. Returns bytes consumed, or None when the
/// char does not start a string.
fn next_token_end(rest: &str) -> Option<usize> {
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        // identifier/keyword/space: skip one char (conservative scan)
        return Some(quote.len_utf8());
    }
    let bytes = rest.as_bytes();
    let mut i = quote.len_utf8();
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2; // skip escaped char
            continue;
        }
        if bytes[i] == quote as u8 {
            return Some(i + 1);
        }
        i += 1;
    }
    Some(rest.len()) // unterminated: consume the rest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::langs::{by_extension, LANGUAGES};

    fn lang(name: &str) -> &'static LangDef {
        LANGUAGES.iter().find(|l| l.name == name).unwrap()
    }

    #[test]
    fn languages_table_sane() {
        assert!(LANGUAGES.len() >= 30);
        for l in LANGUAGES {
            assert!(!l.extensions.is_empty(), "{} has no extensions", l.name);
        }
        assert!(by_extension("rs").is_some());
        assert!(by_extension("py").unwrap().name == "Python");
    }

    #[test]
    fn rust_block_comments() {
        let src = "fn main() {\n    /* block\n       still block */\n    let x = 1; // trailing\n}\n";
        let c = count_lines(src, lang("Rust"));
        assert_eq!((c.code, c.comments, c.blanks), (3, 2, 0), "{c:?}");
    }

    #[test]
    fn comment_opener_inside_string_is_code() {
        let src = "let url = \"http://example.com\"; // docs\n";
        let c = count_lines(src, lang("Rust"));
        assert_eq!(c.code, 1);
        assert_eq!(c.comments, 0);
    }

    #[test]
    fn python_hash_comments() {
        let src = "# shebang-ish\ndef f():\n    return 1  # trailing\n\n";
        let c = count_lines(src, lang("Python"));
        assert_eq!((c.code, c.comments, c.blanks), (2, 1, 1), "{c:?}");
    }

    #[test]
    fn html_block_comments() {
        let src = "<!-- nav -->\n<p>hi</p>\n<!--\nmulti\n-->\n";
        let c = count_lines(src, lang("HTML"));
        assert_eq!((c.code, c.comments, c.blanks), (1, 4, 0), "{c:?}"); // closing --> line counts as comment
    }

    #[test]
    fn sql_double_dash() {
        let src = "-- header\nSELECT 1; -- body\n";
        let c = count_lines(src, lang("SQL"));
        assert_eq!((c.comments, c.code), (1, 1));
    }

    #[test]
    fn blank_lines_counted_once_block_open() {
        let src = "/* start\n\n\nend */\ncode\n";
        let c = count_lines(src, lang("C"));
        assert_eq!(c.blanks, 0); // blanks inside blocks count as comments
        assert_eq!(c.comments, 4);
        assert_eq!(c.code, 1);
    }
}
