use crate::board::Board;

struct Node {
    board: Board,
    fitness: u64,
    action: String,
}

impl Node {
    fn new(board: Board) -> Self {
        Node {
            board: board.clone(),
            fitness: get_fitness(&board.board),
            action: String::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            board: self.board.clone(),
            fitness: self.fitness,
            action: self.action.clone(),
        }
    }
}

pub fn get_fitness(grid: &[u8; 16]) -> u64 {
    let powers: [u8; 16] = [0, 0, 0, 0, 4, 2, 0, 0, 6, 8, 10, 12, 20, 18, 16, 14];

    grid.iter().zip(&powers).fold(0, |acc, (&x, &y)| {
        acc + if (x > 0) & (y > 0) { 1 << (x + y) } else { 0 }
    })
}

pub fn beam_search(board: &Board) -> String {
    let mut queue_a: Vec<Node> = Vec::new();
    let mut queue_b: Vec<Node> = Vec::new();

    let root = Node::new(board.clone());
    queue_a.push(root.clone());

    let mut best_node = root.clone();

    while !queue_a.is_empty() {
        for node in queue_a {
            if node.board.score > best_node.board.score {
                best_node = node.clone();
            }
            for i in 1..5 {
                let mut new_board = node.board.clone();
                let moved = new_board.play(i);
                if moved {
                    let mut new_node = Node::new(new_board);
                    new_node.action = node.action.clone()
                        + match i {
                            1 => "U",
                            2 => "L",
                            3 => "D",
                            4 => "R",
                            _ => "",
                        };
                    queue_b.push(new_node);
                }
            }
        }

        queue_b.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        // queue_b.reverse();
        queue_b.truncate(100);
        queue_a = queue_b;
        queue_b = Vec::new();
    }

    best_node.action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fitness() {
        let board = [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2];
        let expected = (1 << 25) + (1 << 22) + (1 << 19) + (1 << 16) + (1 << 3);
        let fitness = get_fitness(&board);
        assert_eq!(fitness, expected);
    }
}
