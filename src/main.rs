mod path;

use path::VirtualPathBuf;
use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    let mut cwd = VirtualPathBuf::root();

    loop {
        match rl.readline(&format!("limo:{} $ ", cwd)) {
            Ok(line) => {
                if let Some((cmd, args)) = line.split_once(' ') {
                    match cmd {
                        "cd" => {
                            cwd.push(VirtualPathBuf::from(args));
                        },
                        _ => {
                            println!("Unknown command '{}'", cmd)
                        },
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {},
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
