use std::path::Path;
use std::fs::read_to_string;

fn main() {
    let git_dir   = Path::new(".git");
    let head_path = git_dir.join("HEAD");

    let head_ref = read_to_string(head_path).unwrap();

    print!("{}", head_ref);
}
