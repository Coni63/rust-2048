pub use game2048lib::board;

#[test]
fn test_board() {
    let mut game = board::Board::new(290797);

    assert!(game.seed == 13339144);
    assert!(game.board == [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 4, 0, 0]);

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

    assert!(game.board == [8, 2, 8, 4, 2, 16, 32, 8, 16, 2, 8, 16, 4, 16, 4, 2]);
    assert!(game.score == 280);
    assert!(game.is_game_over());
}
