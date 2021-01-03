use clap::Clap;
use engine::db::{Database, LogicDatabase, LoweringDatabase};
use error::LogicResult;
use parse::ast;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let src =
        if let Some(path) = &opts.path { std::fs::read_to_string(path)? } else { String::new() };
    let db = Database::new(src);
    repl(db)?;
    Ok(())
}

fn repl(db: Database) -> Result<(), Box<dyn std::error::Error>> {
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
                solve(&db, goal)?;
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

fn solve(db: &Database, goal: ast::Goal) -> LogicResult<()> {
    ir::tls::set_debug_ctxt(Box::new(ir::IRInterner));
    let env = db.env()?;
    dbg!(&env);
    engine::RecursiveSolver { env, interner: ir::IRInterner };
    Ok(())
}
