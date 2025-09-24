use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Config, Context, Editor, Helper, error::ReadlineError};

#[derive(Clone)]
pub struct MyHelper;

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        Ok((0, vec![])) // Stub for tab-completion
    }
}

impl Hinter for MyHelper {
    type Hint = String;
}
impl Highlighter for MyHelper {}
impl Validator for MyHelper {}
impl Helper for MyHelper {}

pub struct LineEditor {
    rl: Editor<MyHelper, DefaultHistory>, // Corrected type and name
}

impl LineEditor {
    pub fn new() -> Self {
        let config = Config::builder().history_ignore_dups(true).unwrap().build();

        let helper = MyHelper;
        let mut rl = Editor::with_config(config).expect("Failed to create Editor");
        rl.set_helper(Some(helper));
        rl.load_history(".history.txt").ok(); // Optional: load history

        LineEditor { rl }
    }

    pub fn read_line(&mut self) -> Result<String, ReadlineError> {
        self.rl.readline(">> ")
    }

    pub fn add_history_entry(&mut self, line: &str) {
        if let Err(e) = self.rl.add_history_entry(line) {
            eprintln!("Failed to add history entry: {}", e);
        }
        self.rl.save_history(".history.txt").ok(); // Optional: save history
    }
}
