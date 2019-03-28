use std::path::Path;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct GitParseError;

fn main() {
    let git_dir   = Path::new(".git");
    let head_path = git_dir.join("HEAD");

    let head_ref = read_to_string(head_path).unwrap();

    let branch_path = Path::new(resolve_indirect_ref(&head_ref).unwrap());

    print!("{}", branch_path.file_name().unwrap().to_str().unwrap());

}

fn resolve_indirect_ref(refstring: &String) -> Result<&str, GitParseError> {
    if refstring.starts_with("ref: ") {
        Ok(refstring.trim_start_matches("ref: "))
    } else {
        Err(GitParseError)
    }
}
