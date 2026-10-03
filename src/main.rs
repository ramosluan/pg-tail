use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| "/var/log/postgresql/postgresql.log".into());
    let keep = env::var("LINES").ok().and_then(|s| s.parse().ok()).unwrap_or(100);
    let f = File::open(&path).expect("abrir log");
    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(f).lines().flatten() {
        if line.contains("ERROR") || line.contains("FATAL") || line.contains("slow") {
            lines.push(line);
            if lines.len() > keep { lines.remove(0); }
        }
    }
    for ln in lines { println!("{ln}"); }
}
