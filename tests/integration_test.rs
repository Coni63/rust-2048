pub use game2048lib::board;

#[test]
fn test_board() {
    let mut game = board::Board::new(290797);

    game.add_random_tile();
    game.add_random_tile();

    assert!(game.seed == 13339144);
    assert!(game.board == [0, 0, 0, 0,
                           0, 0, 0, 2,
                           0, 0, 0, 0,
                           0, 4, 0, 0]);
}