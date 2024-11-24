use cli::{print_project_info, Cli};
use handlers::handle_project;

mod cli;
mod handlers;
mod templates;

fn main() {
    let cli = Cli::parse_or_interactive();
    handle_project(&cli);
    print_project_info(&cli);
}
