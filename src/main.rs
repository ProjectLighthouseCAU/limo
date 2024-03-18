mod cmd;
mod context;
mod path;

use std::cell::RefCell;

use anyhow::Result;
use clap::Parser;
use context::Context;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use path::VirtualPathBuf;
use rustyline::{config::Configurer, error::ReadlineError, history::DefaultHistory, Editor};
use tokio::runtime::Handle;
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

    let handle = Handle::current();
    let mut rl = Editor::<Context, DefaultHistory>::new().unwrap();
    rl.set_auto_add_history(true);
    rl.set_helper(Some(Context {
        lh: RefCell::new(Lighthouse::connect_with_tokio_to(&args.url, auth).await?),
        cwd: VirtualPathBuf::root(),
        handle,
    }));

    loop {
        let prompt = format!("{}@{}:{} $ ", args.username, host, rl.helper().unwrap().cwd);
        match rl.readline(&prompt) {
            Ok(line) => {
                let (cmd, args) = line.split_once(' ').unwrap_or_else(|| (line.as_ref(), ""));
                let ctx = rl.helper_mut().unwrap();
                let result = cmd::interpret(cmd, args, ctx).await;
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
