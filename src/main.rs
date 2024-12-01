use mdbook_autolink::AutoLink;

use clap::{crate_version, Arg, Command};
use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};
use std::io;

pub fn make_app() -> Command {
    Command::new("mdbook-autolink")
        .version(crate_version!())
        .about("mdbook preprocessor to add wiki-style links")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn handle_preprocessing(pre: impl Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = make_app().get_matches();
    if let Some(_sub_args) = matches.subcommand_matches("supports") {
        Ok(())
    } else {
        handle_preprocessing(AutoLink)
    }
}
