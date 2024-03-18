mod cmd;
mod context;
mod path;

use anyhow::Result;
use clap::Parser;
use context::Context;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use path::VirtualPathBuf;
use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};
use url::Url;

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

    let url = Url::parse(&args.url)?;
    let host = url.host_str().unwrap_or("?");

    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    let mut ctx = Context {
        lh: Lighthouse::connect_with_tokio_to(&args.url, auth).await?,
        cwd: VirtualPathBuf::root(),
    };

    loop {
        let prompt = format!("{}@{}:{} $ ", args.username, host, ctx.cwd);
        match rl.readline(&prompt) {
            Ok(line) => {
                let (cmd, args) = line.split_once(' ').unwrap_or_else(|| (line.as_ref(), ""));
                let result = cmd::interpret(cmd, args, &mut ctx).await;
                if let Err(e) = result {
                    println!("{}", e);
                };
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
