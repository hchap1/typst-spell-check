use std::fs::read_to_string;
use std::env::args;
use regex::Regex;

fn remove_equations(input: &str) -> String {
    let re = Regex::new(r"\$.*?\$").unwrap();
    re.replace_all(input, "").to_string()
}

fn remove_punctuation(input: &str) -> String {
    let re = Regex::new(r"[,.'-][^\s]").unwrap();
    re.replace_all(input, "").to_string().to_lowercase()
}

fn main() {
    let mut args = args();
    match args.len() {
        2 => {
            let filepath: String = args.nth(1).unwrap();
            match read_to_string(&filepath) {
                Ok(raw) => {
                    let document: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                    let doc_string: String = remove_punctuation(&remove_equations(&document.join(" ")));
                    println!("{doc_string}");
                }
                Err(_) => {
                    eprintln!("Could not read '{filepath}'", );
                }
            }
        }
        _ => {
            eprintln!("Expected typst-spell-check <FILENAME>");
        }
    }
}
