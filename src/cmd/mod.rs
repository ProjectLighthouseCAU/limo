use anyhow::{bail, Result};

use crate::context::Context;

macro_rules! cmd_mods {
    ($($mod:ident),* $(,)?) => {
        $(mod $mod;)*

        pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
            Ok(match args[0].as_str() {
                $(stringify!($mod) => $mod::invoke(args, ctx).await?,)*
                "help" => bail!("Available commands: {}", [$(stringify!($mod),)*].join(", ")),
                cmd => bail!("Unrecognized command: {}", cmd),
            })
        }
    };
}

cmd_mods! {
    cat,
    cd,
    cp,
    display,
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
}
