pub use game2048lib::board;

#[test]
fn test_board() {
    let mut game = board::Board::new(290797);

    assert!(game.seed == 13339144);
    assert!(game.board == [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 0]);

    for letter in "DRDURRLRDUURUDRRDDRDLULRDULUDRULLUDDLRDLDLL".chars() {
        let action = match letter {
            'U' => 1,
            'L' => 2,
            'D' => 3,
            'R' => 4,
            _ => 0,
        };

        let moved = game.play(action);
        assert!(moved);
    }

    assert!(game.board == [3, 1, 3, 2, 1, 4, 5, 3, 4, 1, 3, 4, 2, 4, 2, 1]);
    assert!(game.score == 280);
    assert!(game.is_game_over());
}
