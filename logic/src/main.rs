use clap::Clap;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let db = if let Some(path) = &opts.path {
        let src = std::fs::read_to_string(path)?;
        engine::Database::new(src)
    } else {
        engine::Database::default()
    };

    repl(db)?;
    Ok(())
}

fn repl(db: engine::Database) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline("?- ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                let goal = match parse::parse_goal(&line) {
                    Ok(goal) => goal,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                // db.solve(&goal)?;
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}

pub type LogicResult<T> = Result<T, String>;

fn solve(src: &str) -> LogicResult<()> {
    let goal = parse::parse_goal(&src)?;
    println!("{}", goal);
    Ok(())
}
