use std::fs::read_to_string;
use std::env::args;
use regex::Regex;

fn remove_equations(input: &str) -> String {
    let re = Regex::new(r"\$.*?\$").unwrap();
    re.replace_all(input, "").to_string()
}

fn remove_punctuation(input: &str) -> String {
    let re = Regex::new(r"[,\.'-:][^\s]*").unwrap();
    re.replace_all(input, "").to_string().to_lowercase()
}

fn remove_double_spaces(input: &str) -> String {
    let re = Regex::new(r"  ").unwrap();
    re.replace_all(input, " ").to_string()
}

fn remove_parenthesis(input: &str) -> String {
    let re = Regex::new(r"\([^()]*\)").unwrap();
    let mut result = input.to_string();
    while re.is_match(&result) {
        result = re.replace_all(&result, "").to_string();
    }
    result
}

fn remove_functions(input: &str) -> String {
    let re = Regex::new(r"#\S+").unwrap();
    re.replace_all(input, "").to_string()
}

fn main() {
    let mut args = args();
    match args.len() {
        2 => {
            let filepath: String = args.nth(1).unwrap();
            match read_to_string(&filepath) {
                Ok(raw) => {
                    let document: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                    let mut doc_string: String = remove_parenthesis(&document.join(" "));
                    doc_string = remove_equations(&doc_string);
                    doc_string = remove_parenthesis(&doc_string);
                    doc_string = remove_functions(&doc_string);
                    doc_string = remove_punctuation(&doc_string);
                    doc_string = remove_double_spaces(&doc_string);
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
