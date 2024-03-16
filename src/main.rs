mod path;

use path::{VirtualPathBuf, ABS};
use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    let mut cwd = VirtualPathBuf::<ABS>::new();

    loop {
        match rl.readline(&format!("limo:{} $ ", cwd)) {
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
