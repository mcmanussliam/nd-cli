mod choice;
mod cli;
mod init;
mod navigation;
mod view;

use clap::Parser;
use std::{
    error::Error,
    io::{self, Write},
};

use choice::{Choice, commands_hint, out_of_range_message};
use cli::{Args, Command};
use init::init_script;
use navigation::list_directories;
use view::View;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(Command::Init { shell }) = args.command {
        println!("{}", init_script(shell));
        return Ok(());
    }

    if let Some(start_dir) = args.start_dir {
        std::env::set_current_dir(&start_dir)?;
    }

    let use_color = !args.no_color && std::env::var_os("NO_COLOR").is_none();
    let view = View::new(use_color);

    loop {
        view.clear();

        let current_dir = std::env::current_dir()?;
        let choices = list_directories(&current_dir, args.show_hidden)?;

        view.muted(&format!("Path: {}", current_dir.display()));
        view.muted(&format!("{}\n", commands_hint(choices.len())));

        if choices.is_empty() {
            view.muted("No subdirectories found.");
        } else {
            for (index, choice) in choices.iter().enumerate() {
                view.row(index + 1, &choice.name);
            }
        }

        let choice = prompt_choice(&view, choices.len())?;

        match choice {
            Choice::Quit => {
                view.clear();
                println!("{}", current_dir.display());
                io::stdout().flush()?;
                return Ok(());
            }
            Choice::Back => {
                if let Some(parent) = current_dir.parent() {
                    std::env::set_current_dir(parent)?;
                }
            }
            Choice::Select(index) => {
                if let Some(selected) = choices.get(index - 1) {
                    std::env::set_current_dir(&selected.path)?;
                } else {
                    view.warn(&out_of_range_message(choices.len()));
                }
            }
        }
    }
}

fn prompt_choice(view: &View, max_choice: usize) -> io::Result<Choice> {
    loop {
        view.prompt("> ");
        io::stdout().flush()?;

        let mut raw = String::new();
        io::stdin().read_line(&mut raw)?;

        match raw.parse::<Choice>() {
            Ok(Choice::Select(value)) if value > max_choice => {
                view.warn(&out_of_range_message(max_choice));
            }
            Ok(choice) => return Ok(choice),
            Err(_) => view.warn("Enter a number, [b], or [q]."),
        }
    }
}
