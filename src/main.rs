use clap_complete::{generate, Generator, Shell};
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, clap::Parser)]
#[clap(rename_all = "kebab-case")]
struct Common {
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq, clap::Subcommand)]
#[clap(rename_all = "kebab-case")]
enum Sub {
    Comp {
        #[clap(long)]
        shell: Shell,
    },
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(flatten)]
    pub common: Common,

    #[clap(subcommand)]
    pub command: Option<Sub>,
}

fn print_completions<G: Generator>(gen: G) {
    let mut app = <Args as clap::CommandFactory>::command();
    let app = &mut app;
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    match args.command {
        Some(Sub::Comp { shell }) => {
            print_completions(dbg!(shell));
        }
        None => {
            println!("default: {:?}", args.common);
        }
    }
}
