use std::fs::File;
use std::io;
use std::io::BufRead;

fn split(source: String) -> Result<(), String> {
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().collect();
    let mut scope = 0usize;
    for line in lines {
        let whitespace_front = line.len() - line.trim_start().len();
        if whitespace_front % 4 != 0 {
            return Err("whitespace fail".to_string());
        }
        let this_scope = whitespace_front / 4;
        if this_scope == scope - 1 {
            // finish block
            scope -= 1;
        } else if this_scope != scope {
            return Err("wrong scope".to_string());
        }
    }
    Ok(())
}
