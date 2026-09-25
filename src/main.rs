use std::{env, fs, process};
use std::io::Read;

fn run_program(path: &str) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some(value) = line.strip_prefix("print ") {
            println!("{}", unquote(value.trim()));
        } else if line == "pause" {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).ok();
        } else {
            return Err(format!("unknown RET command: {line}"));
        }
    }
    Ok(())
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\''))) {
        s[1..s.len()-1].replace("\\n", "\n").replace("\\"", """)
    } else { s.to_string() }
}

fn main() {
    let mut args = env::args().skip(1);
    let file = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("RET Runtime 0.1");
            eprintln!("Usage: ret <program.ret>");
            process::exit(2);
        }
    };
    if let Err(e) = run_program(&file) {
        eprintln!("RET error: {e}");
        process::exit(1);
    }
}
