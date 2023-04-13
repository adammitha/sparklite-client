use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, Editor};
use sparklite_client::{Client, FilterPredicate};
use std::str::FromStr;

static HELP_TEXT: &'static str = r#"
The following commands are available:
help: print the help text
create <dataset_id> <file>: Create a new dataset from a file
load <dataset_id>: Ask the server to load the specified dataset for manipulation
get <dataset_id>: Download the specified dataset with any transformations that you've applied
filter <predicate> <argument>: Filter the dataset with the specified predicate and argument. Currently we only support the 'eq' predicate applied to the id column of your dataset.
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // let res = client.load_data("test_data").await.unwrap();
    // println!("{:?}", res);
    // println!("{:?}", client.filter("test_data", sparklite_client::FilterPredicate::Eq("abc".into())).await.unwrap());
    // println!("{:?}", client.load_data("123").await.unwrap());
    let mut cli = Cli::new(DefaultEditor::new()?);
    cli.repl().await;
    Ok(())
}

struct Cli {
    rl: Editor<(), FileHistory>,
    dataset_id: Option<String>,
    client: Client<hyper::client::HttpConnector>,
}

impl Cli {
    fn new(rl: Editor<(), FileHistory>) -> Self {
        Self {
            rl,
            dataset_id: None,
            client: sparklite_client::Client::new(
                "http://localhost:8000".parse().unwrap(),
                hyper::client::HttpConnector::new(),
            ),
        }
    }

    async fn repl(&mut self) {
        println!("Welcome to the SparkLite client! Start by loading a dataset");
        loop {
            let readline = self.rl.readline(">> ");
            match readline {
                Ok(line) => match line.parse::<Command>() {
                    Ok(c) => println!("{}", self.execute_command(c).await.unwrap()),
                    Err(err) => println!("{}", err),
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    return;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    return;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    return;
                }
            }
        }
    }

    async fn execute_command(&self, comm: Command) -> anyhow::Result<String> {
        match comm {
            Command::Help => Ok(String::from(HELP_TEXT)),
            Command::Create(_) => todo!(),
            Command::Load(id) => Ok(self.client.load_data(&id).await?),
            Command::Filter(pred) => {
                if let Some(id) = &self.dataset_id {
                    Ok(self.client.filter(id, pred).await?)
                } else {
                    Err(anyhow::anyhow!(
                        "You must load a dataset before running any transformations!"
                    ))
                }
            }
            Command::Get(_) => todo!(),
        }
    }
}

enum Command {
    Help,
    Create(String),
    Load(String),
    Filter(FilterPredicate),
    Get(String),
}

#[derive(thiserror::Error, Debug)]
enum ParseCommandErr {
    #[error("Invalid syntax")]
    SyntaxError(String),
    #[error("Invalid command")]
    InvalidCommand,
}

impl FromStr for Command {
    type Err = ParseCommandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(" ").collect::<Vec<&str>>();
        let command_str = words.get(0).ok_or(ParseCommandErr::SyntaxError(
            "Command must contain at least 1 word".into(),
        ))?;
        let arg_str = words.get(1);

        if command_str.eq_ignore_ascii_case("help") {
            return Ok(Self::Help);
        } else if command_str.eq_ignore_ascii_case("create") {
            if let Some(&s) = arg_str {
                return Ok(Self::Create(s.into()));
            }
            return Err(ParseCommandErr::SyntaxError(
                "You need to specify the name of your dataset".into(),
            ));
        } else if command_str.eq_ignore_ascii_case("load") {
            if let Some(&s) = arg_str {
                return Ok(Self::Load(s.into()));
            }
            return Err(ParseCommandErr::SyntaxError(
                "You need to specify the name of your dataset".into(),
            ));
        } else if command_str.eq_ignore_ascii_case("get") {
            if let Some(&s) = arg_str {
                return Ok(Self::Get(s.into()));
            }
            return Err(ParseCommandErr::SyntaxError(
                "You need to specify the name of your dataset".into(),
            ));
        } else if command_str.eq_ignore_ascii_case("filter") {
            if let (Some(&pred), Some(&arg)) = (arg_str, words.get(2)) {
                if pred == "eq" {
                    return Ok(Self::Filter(FilterPredicate::Eq(arg.into())));
                }
                return Err(ParseCommandErr::InvalidCommand);
            }
            return Err(ParseCommandErr::SyntaxError(
                "You need to specify a predicate and argument to filter your dataset".into(),
            ));
        }
        Err(ParseCommandErr::InvalidCommand)
    }
}
