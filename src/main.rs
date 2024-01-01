mod board;
mod action;


fn main() {
    let mut game = board::Board::new();
    game.add_random_tile();

    loop {
        let action = action::get_action();
        println!("{}", action);

        game.apply_action(action);
        
        if game.is_game_over() {
            println!("Game over!");
            break;
        }

        game.add_random_tile();

        game.print();
    }
    // game.set(0, 0, 2);

    // let mut game2 = game.copy();

    // game.set2(12, 4);

    // game.print();

    // game2.print();
}
