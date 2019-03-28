use std::path::Path;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct ParseError;

fn main() {
    let git_dir   = Path::new(".git");
    let head_path = git_dir.join("HEAD");

    let head_ref = read_to_string(head_path).unwrap();

    print!("{}", resolve_indirect_ref(&head_ref).unwrap());
}

fn resolve_indirect_ref(refstring: &String) -> Result<&str, ParseError> {
    if refstring.starts_with("ref: ") {
        Ok(refstring.trim_start_matches("ref: "))
    } else {
        Err(ParseError)
    }
}
