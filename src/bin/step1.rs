extern crate rustyline;

use mal_rust::{printer, reader};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("mal-rs> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                rl.save_history(".mal-history")?;
                if line.len() > 0 {
                    match reader::read_str(&line) {
                        Ok(mv) => {
                            println!("{}", printer::pr_str(&mv))
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
