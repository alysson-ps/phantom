use clap::{Args, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct PhantomCliCommands {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Run config initialization wizard
    Init(InitArgs),

    /// Run the parser
    Parser(ParserArgs),
}

#[derive(Args, Debug, Clone)]
struct InitArgs {
    #[arg(short, long, help = "Skip confirmation prompt")]
    yes: bool,
}

#[derive(Args, Debug, Clone)]
struct ParserArgs {
    path: String,

    #[arg(long, help = "Apply fixes to the code")]
    fix: bool,

    #[arg(long, help = "Apply fixes to the code without save the changes")]
    fix_dry_run: bool,

    #[arg(long, help = "Output debugging information", action = clap::ArgAction::Count , default_value = "0")]
    debug: u8,
}

#[derive(Debug, Clone, EnumIter, AsRefStr)]
enum Framework {
    Laravel,
    CakePHP,
    Symfony,
    Php,
}

fn initialize(args: &InitArgs) {
    dbg!(args);

    let frameworks: Vec<String> =
        Framework::iter().map(|f| f.as_ref().to_string().to_lowercase()).collect();

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which framework does your project use?")
        .default(0)
        .items(&frameworks)
        .interact()
        .unwrap();

    let _framework = Framework::iter().nth(selected).unwrap();

    // Config {
    //     framework: Some(framework),
    // }
}

fn parser(args: &ParserArgs) {
    // dbg!(args);

    let content = std::fs::read_to_string(&args.path).expect("Failed to read file");

    let result = phantom_parser::parse(&content);

    dbg!(&result);
}

fn main() {
    let args = PhantomCliCommands::parse();

    let result = match &args.command {
        Commands::Init(args) => {
            initialize(args);
        }
        Commands::Parser(args) => {
            parser(args);
        }
    };

    dbg!(result);
    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("Failed to read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // let source = std::fs::read_to_string("examples/sem_erros.php").unwrap();

    // let result = phantom_parser::parser(&source);

    // dbg!(&result);
}
