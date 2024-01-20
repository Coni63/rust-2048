pub mod agent;
pub mod board;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[allow(dead_code)]
fn main() {
    let mut game = board::Board::new(290797);

    println!("{:#}", game);

    let start = std::time::Instant::now();
    let ans = agent::beam_search(&game);
    let elapsed = start.elapsed();

    println!("Actions: {}", ans);
    println!("Time: {:?}", elapsed.as_millis());

    // let mut rng = StdRng::seed_from_u64(222);
    // let die = Uniform::from(1..5);

    // let mut s = String::new();

    // loop {
    //     let action = die.sample(&mut rng);
    //     let moved = game.play(action);

    //     if !moved {
    //         continue;
    //     }

    //     s += match action {
    //         1 => "U",
    //         2 => "L",
    //         3 => "D",
    //         4 => "R",
    //         _ => "",
    //     };

    //     if game.is_game_over() {
    //         println!("Game over!");
    //         break;
    //     }
    // }
    // println!("{:#}", game); // Debug --> same for now as Display
    // println!("Actions: {}", s);
}
