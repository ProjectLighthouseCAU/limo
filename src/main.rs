mod cmd;
mod context;
mod path;

use anyhow::Result;
use clap::Parser;
use context::Context;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use path::VirtualPathBuf;
use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor};
use tokio::fs;
use url::Url;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    /// Interpret/run the given command line.
    #[arg(short)]
    command: Option<String>,
    /// Path to a shell script to interpret.
    script_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);

    let url = Url::parse(&args.url)?;
    let host = url.host_str().unwrap_or("?");

    let ctx = Context {
        lh: Lighthouse::connect_with_tokio_to(&args.url, auth).await?,
        cwd: VirtualPathBuf::root(),
        host: host.to_string(),
        username: args.username,
    };

    if let Some(command) = args.command {
        run_script(&command, ctx).await
    } else if let Some(script_path) = args.script_path {
        let script = fs::read_to_string(script_path).await?;
        run_script(&script, ctx).await
    } else {
        run_interactive(ctx).await
    }
}

async fn run_interactive(mut ctx: Context) -> Result<()> {
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    println!("Limo {} (interactive shell)", VERSION);

    loop {
        let prompt = format!("{}@{}:{} $ ", ctx.username, ctx.host, ctx.cwd);
        match rl.readline(&prompt) {
            Ok(line) => {
                let result = cmd::interpret_line(&line, &mut ctx).await;
                if let Err(e) = result {
                    println!("{}", e.to_string().trim());
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

async fn run_script(script: &str, mut ctx: Context) -> Result<()> {
    for line in script.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue
        }
        cmd::interpret_line(line, &mut ctx).await?;
    }
    Ok(())
}

