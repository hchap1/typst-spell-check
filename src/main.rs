use std::fs::{create_dir_all, read_to_string};
use std::env::{var, args};
use regex::Regex;
use std::collections::HashSet;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct Dictionary {
    dict_words: HashSet<String>
}

impl Dictionary {
    fn new(filepath: &Path) -> Result<Dictionary, String> {
        match read_to_string(&filepath) {
            Ok(raw) => {
                let mut dictionary: Dictionary = Dictionary { dict_words: HashSet::new() };
                let lines: Vec<String> = raw.lines().map(|x| x.to_string().to_lowercase()).collect::<Vec<String>>();
                for line in lines {
                    dictionary.dict_words.insert(line);
                }
                Ok(dictionary)
            }
            Err(_) => {
                Err(format!("Could not access dictionary.txt in APPDATA."))
            }
        }
    }

    fn check(&self, word: &String) -> bool {
        self.dict_words.contains(word)
    }
}

fn remove_equations(input: &str) -> String {
    // let re = Regex::new(r"\\[^$]([^\$]*)\$").unwrap(); <- Non-functional, has edge cases.
    let re = Regex::new(r"\$.*\$").unwrap();
    re.replace_all(input, "").to_string()
}

fn remove_punctuation(input: &str) -> String {
    let re = Regex::new(r#"[\|\?;\[\]<>=",\.'-:][^\s]*"#).unwrap();
    re.replace_all(input, "").to_string().to_lowercase()
}

fn remove_double_spaces(input: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
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
    let re = Regex::new(r"[#@<][\S]+").unwrap();
    re.replace_all(input, "").to_string()
}

fn pattern_println(input: &String, patterns: &Vec<String>) {
    let regex_pattern = patterns.join("|");
    let regex = Regex::new(&regex_pattern).unwrap();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut last_end = 0;
    for mat in regex.find_iter(input) {
        if last_end < mat.start() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
            print!("{}", &input[last_end..mat.start()]);
        }

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        print!("{}", mat.as_str());

        last_end = mat.end();
    }

    if last_end < input.len() {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
        print!("{}", &input[last_end..]);
    }

    stdout.reset().unwrap();
    print!("\n");
}

fn match_lines(text_lines: Vec<String>, patterns: Vec<String>) -> Vec<Vec<String>> {
    let regexes: Vec<Regex> = patterns
        .iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();
    let mut result: Vec<Vec<String>> = Vec::new();

    for line in text_lines {
        let mut matched_patterns = Vec::new();
        for regex in &regexes {
            if regex.is_match(&line) {
                matched_patterns.push(regex.as_str().to_string());
            }
        }
        result.push(matched_patterns);
    }
    result
}

fn main() {
    let appdata = match var("APPDATA") {
        Ok(path) => {
            path
        }
        Err(_) => {
            panic!("No APPDATA ENV-VAR!");
        }
    };
    let dictpath = Path::new(&appdata).join("typst-spell-check/dictionary.txt");
    if let Some(parent) = dictpath.parent() {
        if let Err(e) = create_dir_all(parent) {
            panic!("Failed to create directory: {}", e);
        }
    }
    match dictpath.exists() {
        true => { }
        false => {
            eprintln!("No dictionary.txt file.");
            let url = "https://raw.githubusercontent.com/hchap1/dictionary_file/refs/heads/main/dictionary.txt";
            match get(url) {
                Ok(response) => {
                    println!("Pulling down dictionary.txt.");
                    if response.status().is_success() {
                        let mut file = File::create(&dictpath).unwrap();
                        let content = response.bytes().unwrap();
                        let _ = file.write_all(&content);
                        println!("Succesfully pulled down dictionary.txt!");
                    } else { eprintln!("Failed to pull down dictionary.txt: {:?}", response.status().canonical_reason()); }
                }
                Err(_) => {
                    eprintln!("Cannot pull down dictionary.txt - check internet.");
                }
            }
        }
    }
    let dictionary: Dictionary = Dictionary::new(&dictpath).unwrap();
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
                    let fmt_document = doc_string.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                    let mut mispelt: Vec<String> = vec![];
                    for word in &fmt_document {
                        match dictionary.check(word) {
                            true => {}
                            false => {
                                if word != "" { mispelt.push(word.clone()); }
                            }
                        }
                    }
                    for line in &document {
                        pattern_println(line, &mispelt);
                    }
                }
                Err(_) => {
                    eprintln!("Could not read '{filepath}'", );
                }
            }
        }
        3 => {
            let filepath: String = args.nth(1).unwrap();
            match read_to_string(&filepath) {
                Ok(raw) => {
                    if args.nth(0).unwrap() == String::from("--report") {
                        let document: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                        let mut doc_string: String = remove_parenthesis(&document.join(" "));
                        doc_string = remove_equations(&doc_string);
                        doc_string = remove_parenthesis(&doc_string);
                        doc_string = remove_functions(&doc_string);
                        doc_string = remove_punctuation(&doc_string);
                        doc_string = remove_double_spaces(&doc_string);
                        let fmt_document = doc_string.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                        let mut mispelt: Vec<String> = vec![];
                        for word in &fmt_document {
                            match dictionary.check(word) {
                                true => {}
                                false => {
                                    if word != "" { mispelt.push(word.clone()); }
                                }
                            }
                        }
                        let errors: Vec<Vec<String>> = match_lines(document, mispelt);
                        println!("Checked {} words.", fmt_document.len());
                        if errors.len() == 0 {
                            println!("All clear! Manually check text in functions and equations.");
                        }
                        for (idx, error) in errors.into_iter().enumerate() {
                            if error.len() != 0 {
                                println!("Line {idx}: {}", error.join(", "));
                            }
                        }
                    } else {
                        eprintln!("Could not read '{filepath}'", );
                    }
                }
                Err(_) => {

                }
            }
        }
        _ => {
            eprintln!("Expected typst-spell-check <FILENAME>");
        }
    }
}
