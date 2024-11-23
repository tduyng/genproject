use cli::Cli;
use handlers::{handle_deno, handle_nestjs, handle_nodejs};

mod cli;
mod handlers;
mod templates;

fn main() {
    let cli = Cli::parse_or_interactive();
    match cli.project_type.as_str() {
        "nodejs" => handle_nodejs(cli),
        "nestjs" => handle_nestjs(cli),
        "deno" => handle_deno(cli),
        _ => println!("Unsupported project type"),
    }
}
