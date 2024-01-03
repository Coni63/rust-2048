pub mod board;
mod action;

use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;


fn main() {

    let mut game = board::Board::new(290797);

    game.add_random_tile();
    println!("{:#}", game);

    game.add_random_tile();
    println!("{:#}", game);

    let mut rng = StdRng::seed_from_u64(222);
    let die = Uniform::from(1..5);

    let mut s = String::new();

    loop {
        // let action = action::get_action();
        let action = die.sample(&mut rng);

        // game.apply_action(action);
        let moved = game.apply_action(action);
        println!("{} - {}", action, moved);
        
        if !moved {
            continue;
        }
        
        s += match action {
            1 => "U",
            2 => "L",
            3 => "D",
            4 => "R",
            _ => "a",
        };

        game.add_random_tile();
        
        if game.is_game_over() {
            println!("Game over!");
            break;
        }
    }
    println!("{:#}", game); // Debug --> same for now as Display
    println!("Actions: {}", s);
}
