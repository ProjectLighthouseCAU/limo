use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                println!("Got {line}");
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
