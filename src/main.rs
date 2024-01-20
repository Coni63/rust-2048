pub mod agent;
pub mod board;

#[allow(dead_code)]
fn main() {
    let game = board::Board::new(290797);

    println!("{:#}", game);

    let start = std::time::Instant::now();
    let ans = agent::beam_search(&game);
    let elapsed = start.elapsed();

    println!("Actions: {}", ans.action);
    println!("{:#?}", ans.board);
    println!("Time: {:?}", elapsed.as_millis());
}
