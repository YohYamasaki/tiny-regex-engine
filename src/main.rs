use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::helper::DynError;

mod engine;
mod helper;

fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("Invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }
    Ok(())
}

fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..])? {
                println!("matched!!: {line}");
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::do_matching,
        helper::{SafeAdd, safe_add},
    };

    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0;
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        // parse error
        assert!(do_matching("+b", "bbb").is_err());
        assert!(do_matching("*b", "bbb").is_err());
        assert!(do_matching("|b", "bbb").is_err());
        assert!(do_matching("?b", "bbb").is_err());
        // successful parsing + match
        assert!(do_matching("abc|def", "def").unwrap());
        assert!(do_matching("(abc)*", "abcabc").unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd").unwrap());
        assert!(do_matching("abc?", "ab").unwrap());
        // successful parsing + does not match
        assert!(!do_matching("abc|def", "efa").unwrap());
        assert!(!do_matching("(ab|cd)+", "").unwrap());
        assert!(!do_matching("abc?", "acb").unwrap());
    }
}
