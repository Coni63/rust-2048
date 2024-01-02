pub fn get_action() -> u8 {
    println!("Enter a command:");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "a" => std::process::exit(0),
            "z" => return 1,
            "q" => return 2,
            "s" => return 3,
            "d" => return 4,
            _ => println!("Invalid input, retry:"),
        }
    }
}