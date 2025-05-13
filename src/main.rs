pub mod commands;
pub mod contact;
pub mod utils;

use crate::commands::handle_command;

fn main() {
    loop {
        println!("Welcome to Contacts Manager\n1 - Add\n2 - Find\n3 - Delete");
        handle_command();
    }
}
