use clap::{Arg, Command, PossibleValue};
use clap_complete::{generate, Generator, Shell};
use std::io;

fn build_cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .subcommand_required(true)
        .subcommand(
            Command::new("foo").arg(
                Arg::new("foo")
                    .long("foo")
                    .help("foos for real")
                    .possible_value(PossibleValue::new("7"))
                    .possible_value(PossibleValue::new("seven")),
            ),
        )
        .subcommand(
            Command::new("comp").arg(
                Arg::new("shell")
                    .long("shell")
                    .help("shell to generate for")
                    .possible_values(Shell::possible_values()),
            ),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("comp", args)) => {
            if let Ok(shell) = args.value_of_t::<Shell>("shell") {
                let mut cmd = build_cli();
                eprintln!("Generating completion file for {}...", shell);
                print_completions(dbg!(shell), &mut cmd);
                return;
            }
            eprintln!("nope");
        }
        Some(("foo", _args)) => {
            println!("foo");
            return;
        }
        _ => {}
    }
    build_cli().print_long_help().unwrap();
}
