use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct GitParseError;

fn main() {
    let git_dir   = Path::new(".git");
    let head_path = git_dir.join("HEAD");

    let mut head_file = BufReader::new(File::open(head_path).unwrap());

    let mut head_ref = String::new();
    
    head_file.read_line(&mut head_ref).unwrap();

    let head = &head_ref.trim_end().to_string();

    let branch_path = Path::new(resolve_indirect_ref(head).unwrap());

    println!("{}", branch_path.file_name().unwrap().to_str().unwrap());

}

fn resolve_indirect_ref(refstring: &String) -> Result<&str, GitParseError> {
    if refstring.ends_with('\n') {
    }
    if refstring.starts_with("ref: ") {
        Ok(refstring.trim_start_matches("ref: "))
    } else {
        if refstring.len() == 40 {
            Ok(refstring)
        } else {
            Err(GitParseError)
        }
    }
}
