use fancy_regex::Regex;
use std::env;
use std::fs;

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: sentencebreak <input-file>");
        return;
    }
    let input_file = &args[1];

    // Read input text
    let input_text = match fs::read_to_string(input_file) {
        Ok(text) => text,
        Err(err) => {
            println!("Error reading input file: {}", err);
            return;
        }
    };

    // Add line breaks after each sentence
    let output_text = add_line_breaks(&input_text);

    // Write modified text to a new file
    let output_file = format!("{}_modified.txt", input_file);
    match fs::write(&output_file, output_text) {
        Ok(_) => println!("Modified text written to {}", output_file),
        Err(err) => println!("Error writing output file: {}", err),
    }
}

fn add_line_breaks(text: &str) -> String {
    let re = Regex::new(r"(?<!\w\.\w.)(?<![A-Z][a-z]\.)(?<=\.|\?)(?!\s|\se\.g\.)(\s*)").unwrap();
    re.replace_all(text, "$1\n").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_line_breaks() {
        let input_text = "This is a sentence. This is another sentence? This is a third sentence.";
        let expected_output = "This is a sentence.\nThis is another sentence?\nThis is a third sentence.";
        assert_eq!(add_line_breaks(input_text), expected_output);
    }

    #[test]
    fn test_add_line_breaks_with_eg_pattern() {
        let input_text = "This is another with e.g. many periods? This is a third e.g.";
        let expected_output = "This is another with e.g. many periods?\nThis is a third e.g.";
        assert_eq!(add_line_breaks(input_text), expected_output);
    }
}