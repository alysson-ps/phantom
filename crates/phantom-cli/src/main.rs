use clap::{Args, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select};
use phantom_config::load;
use phantom_core::{rich::RichError, token::Token};
use phantom_parser::ParserResult;
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

    /// Manage config file
    Config(ConfigArgs),
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

    // #[arg(long, help = "Output debugging information", action = clap::ArgAction::Count , default_value = "0")]
    #[arg(long, help = "Output debugging information")]
    debug: bool,

    #[arg(short, long, help = "Path to the config file")]
    config: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct ConfigArgs {}

#[derive(Debug, Clone, EnumIter, AsRefStr)]
enum Framework {
    Laravel,
    CakePHP,
    Symfony,
    Php,
}

fn initialize(args: &InitArgs) {
    // dbg!(args);

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
    dbg!(args);

    let content = std::fs::read_to_string(&args.path).expect("Failed to read file");

    let config_path = args.config.clone().unwrap_or(".phantomrc".to_string());
    let ParserResult {
        ast,
        parse_errors,
        tokens,
    } = phantom_parser::parse(&content, &config_path);

    // dbg!(&ast);
    // dbg!(&tokens);

    if args.fix {
        let errs = parse_errors
            .clone()
            .iter()
            .filter(|err| err.fixer().is_some())
            .map(|item| item.clone())
            .collect::<Vec<RichError<Token>>>();

        let formatter = phantom_formatter::Event {
            ast: ast.unwrap(),
            path: &args.path,
            tokens,
            errs,
        };

        phantom_formatter::run(formatter);
    }

    dbg!(parse_errors);
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
        Commands::Config(args) => match load("examples/.phantomrc") {
            Ok(config) => {
                dbg!(&config);
            }
            Err(err) => {
                dbg!(err);
            }
        },
    };

    // dbg!(result);
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
