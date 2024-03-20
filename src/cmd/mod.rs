use anyhow::{bail, Result};

use crate::context::Context;

macro_rules! cmd_mods {
    ($($mod:ident),* $(,)?) => {
        $(mod $mod;)*

        pub async fn invoke(args: &[&str], ctx: &mut Context) -> Result<()> {
            match args[0] {
                $(stringify!($mod) => $mod::invoke(args, ctx).await?,)*
                "help" => println!("Available commands: {}", [$(stringify!($mod),)*].join(", ")),
                cmd => bail!("Unrecognized command: {}", cmd),
            }
            Ok(())
        }
    };
}

cmd_mods!(
    cat,
    cd,
    cp,
    echo,
    ln,
    ls,
    mkdir,
    mv,
    pwd,
    rm,
    rmdir,
    touch,
    tree,
    uln,
);
