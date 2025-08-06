use clap::{Parser, Subcommand};
use meals_lib::handling::{self, CommandHandler};

#[derive(clap::Parser)]
struct Args {
    // #[arg(long, value_parser = clap::value_parser!(MealDay))]
    // days: Vec<MealDay> // should take a path
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate { 
        #[command(subcommand)]
        cmd: GenerateCommand,
    },
}

impl CommandHandler<String> for Command {
    fn handle(&self) -> handling::Result<String> {
        match self {
            Command::Generate { cmd } => cmd.handle()
        }
    }
}

#[derive(Subcommand)]
enum GenerateCommand {
    List { 
        #[arg(long)]
        meals_file: String,
    }
}

// Could we do a derive macro that will generate a func like this
/*
#[derive(Subcommand, Handler)]
enum GenerateCommand {
    List {
        #[arg(long)]
        #[handler(handle_list)]
        meals_file: String,
    }
}

impl GenerateCommand {
    // panic if this func doesn't exist
    // mandates the function having a handler with all the required args
    // but don't create the function? is that allowed?
    // so all its doing is checking if there is an impl, its not really "deriving" anything in the traditional sense
    pub fn handle_list(&self, meals_file: String) -> handling::Result<String> {
    }
}
 */
impl CommandHandler<String> for GenerateCommand {
    fn handle(&self) -> handling::Result<String> {
        match self {
            GenerateCommand::List { meals_file } => {
                Ok(format!("Yay handling! of {}", meals_file).to_string())
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.cmd.handle() {
        Ok(v) => println!("{}", v),
        Err(e) => panic!("{}", e),
    }
}

