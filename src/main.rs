pub mod board;
mod action;


fn main() {

    let mut game = board::Board::new(290797);

    game.add_random_tile();
    println!("{:#}", game);

    game.add_random_tile();
    println!("{:#}", game);

    loop {
        let action = action::get_action();

        game.apply_action(action);
        
        if game.is_game_over() {
            println!("Game over!");
            break;
        }

        game.add_random_tile();

        println!("{:#}", game); // Debug --> same for now as Display
    }
}
