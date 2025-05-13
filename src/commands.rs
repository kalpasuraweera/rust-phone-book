use crate::contact::Contact;
use crate::utils::read_input;
pub fn handle_command() {
    let command: u8 = read_input(String::from("Enter the Command:"))
        .trim()
        .parse()
        .expect("Unexpected Input");
    println!("you entered {}", command);
    match command {
        1 => create_contact(),
        _ => println!("We don't support that command"),
    }
}

fn create_contact() {
    let name: String = read_input(String::from("Enter Name:"));
    let number: String = read_input(String::from("Enter Number:"));
    let new_contact = Contact {
        name: name,
        number: number,
    };
    println!("new name is {}", new_contact.name);
}
