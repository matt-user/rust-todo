use clap::{Args, Parser, Subcommand};
use rusqlite::Error;

use todo_lib::TodoList;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add todo items to app
    Add(AddArgs),
    /// Remove todo items
    Remove(RemoveArgs),
    /// Edit todo items
    Edit(EditArgs),
    /// Mark todo items as done
    Done(DoneArgs),
}

#[derive(Args)]
struct AddArgs {
    todo_items: Vec<String>,
}

#[derive(Args)]
struct RemoveArgs {
    todo_item_indexes: Vec<u64>,
}

#[derive(Args)]
struct EditArgs {
    todo_item_index: u64,
    new_todo_item: String,
}

#[derive(Args)]
struct DoneArgs {
    todo_item_indexes: Vec<u64>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let todo_list = TodoList::new()?;
    todo_list.init_table()?;

    match &cli.command {
        Some(Commands::Add(add_args)) => {
            todo_list.add(&add_args.todo_items)?;
        }
        Some(Commands::Remove(remove_args)) => todo_list.remove(&remove_args.todo_item_indexes)?,
        Some(Commands::Edit(edit_args)) => {
            let EditArgs {
                todo_item_index,
                new_todo_item,
            } = edit_args;
            todo_list.edit(todo_item_index, new_todo_item)?;
        }
        Some(Commands::Done(done_args)) => {
            todo_list.done(&done_args.todo_item_indexes)?;
        }
        None => {
            todo_list.list()?;
        }
    }

    Ok(())
}
