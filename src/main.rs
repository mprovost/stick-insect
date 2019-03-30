use std::path::Path;
use std::fs::{File, metadata};
use std::io::{BufReader, BufRead};
use std::env::{current_dir, set_current_dir};

#[derive(Debug, Clone)]
struct GitParseError;

fn main() {
    if is_inside_work_tree() {
        let git_dir   = Path::new(".git");
        let head_path = git_dir.join("HEAD");

        let mut head_file = BufReader::new(File::open(head_path).unwrap());

        let mut head_ref = String::new();
        
        head_file.read_line(&mut head_ref).unwrap();

        let head = &head_ref.trim_end().to_string();

        let branch_path = Path::new(resolve_indirect_ref(head).unwrap());

        println!("{}", branch_path.file_name().unwrap().to_str().unwrap());
    } else {
        eprintln!("fatal: Not a git repository (or any of the parent directories): .git");
        std::process::exit(128);
    }
}

fn resolve_indirect_ref(refstring: &String) -> Result<&str, GitParseError> {
    if refstring.starts_with("ref: ") {
        Ok(refstring.trim_start_matches("ref: "))
    } else {
        // check for a SHA-1
        if refstring.len() == 40 {
            // just return the first 7 characters of the SHA
            Ok(&refstring[..7])
        } else {
            Err(GitParseError)
        }
    }
}

// git rev-parse --is-inside-work-tree
fn is_inside_work_tree() -> bool {
    let cwd = current_dir().unwrap();

    for wd in cwd.ancestors() {
        //println!("{:?}", wd);
        if is_work_tree(&wd) {
            return true;
        }
    }
    false
}

/*
getcwd("/home/mprovost", 4096)          = 15                                                 
stat(".", {st_mode=S_IFDIR|0755, st_size=4096, ...}) = 0                                     
stat(".git", 0x7ffc3747b3e0)            = -1 ENOENT (No such file or directory)              
access(".git/objects", X_OK)            = -1 ENOENT (No such file or directory)              
access("./objects", X_OK)               = -1 ENOENT (No such file or directory)              
stat("..", {st_mode=S_IFDIR|0755, st_size=4096, ...}) = 0                                    
chdir("..")                             = 0                                                  
stat(".git", 0x7ffc3747b3e0)            = -1 ENOENT (No such file or directory)
*/
fn is_work_tree(wd: &Path) -> bool {
    if let Ok(stat) = metadata(wd) {
        set_current_dir(&wd).unwrap();
        if stat.is_dir() {
            if let Ok(git) = metadata(Path::new(".git")) {
                if git.is_dir() {
                    true
                } else {
                    if metadata(Path::new(".git/objects")).is_ok() {
                        true
                    } else {
                        metadata(Path::new("./objects")).is_ok()
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
