use clap::Clap;
// use logic_engine::Environment;
// use logic_ir::{LogicInterner, LogicResult, Program};
// use logic_parse::{ast, ParseResult};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::Arc;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    // let src =
    //     if let Some(path) = &opts.path { std::fs::read_to_string(path)? } else { String::new() };
    // let db = Database::new(src);
    // repl(db)?;
    Ok(())
}

// fn repl(db: Database) -> Result<(), Box<dyn std::error::Error>> {
//     let mut rl = Editor::<()>::new();
//     let _ = rl.load_history("history.txt");

//     loop {
//         let readline = rl.readline("?- ");
//         match readline {
//             Ok(line) => {
//                 if line.is_empty() {
//                     continue;
//                 }
//                 rl.add_history_entry(line.as_str());
//                 let goal = match logic_parse::parse_goal(&line) {
//                     Ok(goal) => goal,
//                     Err(err) => {
//                         eprintln!("{}", err);
//                         continue;
//                     }
//                 };
//                 dbg!(&goal);
//                 // solve(&db, goal)?;
//             }
//             Err(ReadlineError::Interrupted) => {
//                 break;
//             }
//             Err(ReadlineError::Eof) => {
//                 break;
//             }
//             Err(err) => {
//                 println!("Error: {:?}", err);
//                 break;
//             }
//         }
//     }
//     rl.save_history("history.txt")?;
//     Ok(())
// }

// fn solve(db: &Database, goal: ast::Goal) -> LogicResult<()> {
//     todo!();
//     // tls::set_debug_ctxt(Box::new(IRInterner));
//     // let goal = logic_ir::lower_goal(&goal);
//     // let env = db.env()?;
//     // dbg!(&env);
//     // dbg!(&goal);
//     // let solver = logic_engine::RecursiveSolver::new(IRInterner, env);
//     // dbg!(solver.solve(&goal));
//     Ok(())
// }
