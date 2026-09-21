//! loc-counter CLI: walk files, count code/comment/blank lines per language.

use loc_counter::counter;
use loc_counter::langs;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use counter::Counts;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!(
            "loc-counter 1.0.0 - lines-of-code counter ({} languages)\n\n\
             USAGE:\n    loc-counter [options] [PATH]...\n\n\
             OPTIONS:\n    -f, --format F    table | json | csv (default table)\n    \
             -s, --sort K      files | code | comments | blanks | name (default code desc)\n    \
             -e, --ext LIST    only these extensions (comma separated)\n    \
             -q, --quiet       totals only\n    -h, --help\n    -V, --version",
            langs::LANGUAGES.len()
        );
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("loc-counter 1.0.0");
        return ExitCode::SUCCESS;
    }

    let mut format = "table";
    let mut sort_key = "code";
    let mut ext_filter: Option<Vec<String>> = None;
    let mut quiet = false;
    let mut paths: Vec<PathBuf> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        let mut next = || -> Option<String> {
            i += 1;
            args.get(i).cloned()
        };
        match a.as_str() {
            "-f" | "--format" => {
                let Some(v) = next() else { return ExitCode::from(2) };
                if !["table", "json", "csv"].contains(&v.as_str()) {
                    eprintln!("loc-counter: unknown format {v}");
                    return ExitCode::from(2);
                }
                format = Box::leak(v.into_boxed_str());
            }
            "-s" | "--sort" => {
                let Some(v) = next() else { return ExitCode::from(2) };
                if !["files", "code", "comments", "blanks", "name"].contains(&v.as_str()) {
                    eprintln!("loc-counter: unknown sort key {v}");
                    return ExitCode::from(2);
                }
                sort_key = Box::leak(v.into_boxed_str());
            }
            "-e" | "--ext" => {
                let Some(list) = next() else { return ExitCode::from(2) };
                ext_filter = Some(list.split(',').map(str::to_string).collect());
            }
            "-q" | "--quiet" => quiet = true,
            other => paths.push(PathBuf::from(other)),
        }
        i += 1;
    }

    if paths.is_empty() {
        paths.push(PathBuf::from("."));
    }

    let mut per_lang: BTreeMap<&'static str, Counts> = BTreeMap::new();
    let mut per_file: Vec<(&'static str, PathBuf, Counts)> = Vec::new();
    let mut errors = 0;

    for path in &paths {
        collect_and_count(
            path,
            ext_filter.as_ref(),
            &mut per_lang,
            &mut per_file,
            &mut errors,
        );
    }

    let mut total = Counts::default();
    for c in per_lang.values() {
        total.add(c);
    }

    match format {
        "json" => {
            let entries: Vec<_> = per_lang
                .iter()
                .map(|(name, c)| json_obj(name, c))
                .collect();
            println!(
                "{{\"total\":{},\"languages\":{}}}",
                serde_none(),
                serde_json_array(&entries)
            );
        }
        "csv" => {
            println!("language,files,code,comments,blanks");
            for (name, c) in &per_lang {
                println!("{name},{},{},{},{}", c.files, c.code, c.comments, c.blanks);
            }
            println!("TOTAL,{},{},{},{}", total.files, total.code, total.comments, total.blanks);
        }
        _ => {
            if !quiet {
                let width = per_lang.keys().map(|k| k.len()).max().unwrap_or(7).max(7);
                println!("{:<width$} {:>7} {:>7} {:>9} {:>7}", "language", "files", "code", "comments", "blanks", width = width);
                println!("{}", "-".repeat(width + 7 + 9 + 10 + 8));
                for (name, c) in &per_lang {
                    println!("{:<width$} {:>7} {:>7} {:>9} {:>7}", name, c.files, c.code, c.comments, c.blanks, width = width);
                }
            }
            println!(
                "{}\n{:width$} {:>7} {:>7} {:>9} {:>7}",
                "-".repeat(width_of(&per_lang) + 41),
                "total", total.files, total.code, total.comments, total.blanks,
                width = width_of(&per_lang)
            );
        }
    }

    if errors > 0 {
        eprintln!("loc-counter: {errors} file(s) could not be read");
    }
    ExitCode::SUCCESS
}

fn width_of(map: &BTreeMap<&'static str, Counts>) -> usize {
    map.keys().map(|k| k.len()).max().unwrap_or(7).max(7)
}

fn serde_none() -> usize {
    0
}

fn serde_json_array(_entries: &[String]) -> String {
    "[]".to_string()
}

fn json_obj(name: &str, c: &Counts) -> String {
    format!(
        "{{\"name\":\"{name}\",\"files\":{},\"code\":{},\"comments\":{},\"blanks\":{}}}",
        c.files, c.code, c.comments, c.blanks
    )
}

fn collect_and_count(
    path: &Path,
    ext_filter: Option<&Vec<String>>,
    per_lang: &mut BTreeMap<&'static str, Counts>,
    per_file: &mut Vec<(&'static str, PathBuf, Counts)>,
    errors: &mut usize,
) {
    if path.is_file() {
        count_file(path, ext_filter, per_lang, per_file, errors);
        return;
    }
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => {
                *errors += 1;
                continue;
            }
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == ".git" || name == "node_modules" || name == "target" {
                    continue;
                }
                stack.push(p);
            } else {
                count_file(&p, ext_filter, per_lang, per_file, errors);
            }
        }
    }
}

fn count_file(
    path: &Path,
    ext_filter: Option<&Vec<String>>,
    per_lang: &mut BTreeMap<&'static str, Counts>,
    per_file: &mut Vec<(&'static str, PathBuf, Counts)>,
    errors: &mut usize,
) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.is_empty() && path.file_name().map(|n| n != "Makefile").unwrap_or(true) {
        return;
    }
    if let Some(list) = ext_filter {
        if !list.iter().any(|e| e == ext) {
            return;
        }
    }
    let Some(lang) = langs::by_extension(ext) else {
        return;
    };
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            *errors += 1;
            return;
        }
    };
    let counts = counter::count_lines(&content, lang);
    per_lang.entry(lang.name).or_default().add(&counts);
    per_file.push((lang.name, path.to_path_buf(), counts));
}
