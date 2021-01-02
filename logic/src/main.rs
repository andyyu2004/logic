use clap::Clap;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug, Clap)]
struct Opts {
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let mut rl = Editor::<()>::new();
    let _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline("λ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                solve(&line);
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

fn solve(src: &str) -> Option<()> {
    let goal = parse::parse_goal(&src)?;
    println!("{}", goal);
    Some(())
}
