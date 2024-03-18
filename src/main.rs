mod path;

use anyhow::Result;
use clap::Parser;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use path::VirtualPathBuf;
use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};

#[derive(Parser)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let mut lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;

    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    let mut cwd = VirtualPathBuf::root();

    loop {
        match rl.readline(&format!("limo:{} $ ", cwd)) {
            Ok(line) => {
                let (cmd, args) = line.split_once(' ').unwrap_or_else(|| (line.as_ref(), ""));
                match cmd {
                    "cd" => {
                        cwd.push(VirtualPathBuf::from(args));
                    },
                    "ls" => {
                        let response = lh.list(&cwd.as_relative().as_str_vec()).await?;
                        println!("{}", response.payload);
                    },
                    _ => {
                        println!("Unknown command '{}'", cmd)
                    },
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

    Ok(())
}
