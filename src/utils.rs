use ::std::io;

pub fn read_input(prompt: String) -> String {
    println!("{}", prompt);
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Faild to Read the Commmand");
    command.trim().to_string()
}

