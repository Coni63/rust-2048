pub fn get_action() -> String {
    println!("Enter a command:");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        input = input.trim().to_lowercase().to_string();
    
        if input == "a" {
            std::process::exit(0);
        } else if input == "z" {
            return "up".to_string();
        } else if input == "q" {
            return "left".to_string();
        } else if input == "s" {
            return "down".to_string();
        } else if input == "d" {
            return "right".to_string();
        } else {
            println!("Invalid input, retry:");
        }
    }
}