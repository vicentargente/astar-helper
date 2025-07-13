use std::{fmt::Debug, hash::Hash};

use astar_helper::{astar_state::AStarState, traced::state::TracedState, untraced::state::UntracedState};

const WIDTH: usize = 5;
const HEIGHT: usize = 4;
const NUM_PIECES: u8 = 11;
const TARGET_PIECE: u8 = 1;
const TARGET_POSITION: (usize, usize) = (3, 1);
const BS: u8 = 0xFF; // Blank space

const PIECE_DIMENSIONS: [(usize, usize); NUM_PIECES as usize] = [
    (2, 1),
    (2, 2),
    (2, 1),
    (1, 1),
    (1, 1),
    (1, 1),
    (1, 1),
    (1, 2),
    (1, 2),
    (1, 1),
    (1, 1)
];

fn coordinate_to_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn index_to_coordinate(index: usize) -> (usize, usize) {
    (index % WIDTH, index / WIDTH)
}

#[derive(Clone)]
pub struct Puzzle {
    board: [u8; WIDTH * HEIGHT],
    piece_positions: [(u8, u8); NUM_PIECES as usize],
    current_cost: usize
}

impl Puzzle {
    pub fn new() -> Self {
        Puzzle {
            board: [
                00, 00, 03, 07, 09,
                01, 01, 04, 07, BS,
                01, 01, 05, 08, BS,
                02, 02, 06, 08, 10
            ],
            piece_positions: [
                (0, 0),
                (0, 1),
                (0, 3),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 0),
                (3, 2),
                (4, 0),
                (4, 3)
            ],
            current_cost: 0
        }
    }

    pub fn can_move_piece_up(&self, piece_id: u8) -> bool {
        let (x, y) = self.piece_positions[piece_id as usize];
        if y == 0 {
            return false;
        }

        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize - 1);
            if self.board[index] != BS {
                return false;
            }
        }

        true
    }

    pub fn can_move_piece_down(&self, piece_id: u8) -> bool {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_height = PIECE_DIMENSIONS[piece_id as usize].1;
        if y as usize + piece_height >= HEIGHT {
            return false;
        }

        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize + piece_height);
            if self.board[index] != BS {
                return false;
            }
        }

        true
    }

    pub fn can_move_piece_left(&self, piece_id: u8) -> bool {
        let (x, y) = self.piece_positions[piece_id as usize];
        if x == 0 {
            return false;
        }

        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize - 1, y as usize + v_block as usize);
            if self.board[index] != BS {
                return false;
            }
        }

        true
    }

    pub fn can_move_piece_right(&self, piece_id: u8) -> bool {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_width = PIECE_DIMENSIONS[piece_id as usize].0;
        if x as usize + piece_width >= WIDTH {
            return false;
        }

        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize + piece_width, y as usize + v_block as usize);
            if self.board[index] != BS {
                return false;
            }
        }

        true
    }

    pub fn move_piece_up(&mut self, piece_id: u8) {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_height = PIECE_DIMENSIONS[piece_id as usize].1;

        // Fill the space above with the piece
        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize - 1);
            self.board[index] = piece_id;
        }
        // Clear the current position of the piece
        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize + piece_height - 1);
            self.board[index] = BS;
        }
        self.piece_positions[piece_id as usize].1 -= 1;
    }

    pub fn move_piece_down(&mut self, piece_id: u8) {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_height = PIECE_DIMENSIONS[piece_id as usize].1;

        // Fill the space below with the piece
        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize + piece_height);
            self.board[index] = piece_id;
        }
        // Clear the current position of the piece
        for h_block in 0..PIECE_DIMENSIONS[piece_id as usize].0 {
            let index = coordinate_to_index(x as usize + h_block as usize, y as usize);
            self.board[index] = BS;
        }
        self.piece_positions[piece_id as usize].1 += 1;
    }

    pub fn move_piece_left(&mut self, piece_id: u8) {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_width = PIECE_DIMENSIONS[piece_id as usize].0;

        // Fill the space to the left with the piece
        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize - 1, y as usize + v_block as usize);
            self.board[index] = piece_id;
        }
        // Clear the current position of the piece
        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize + piece_width - 1, y as usize + v_block as usize);
            self.board[index] = BS;
        }
        self.piece_positions[piece_id as usize].0 -= 1;
    }

    pub fn move_piece_right(&mut self, piece_id: u8) {
        let (x, y) = self.piece_positions[piece_id as usize];
        let piece_width = PIECE_DIMENSIONS[piece_id as usize].0;

        // Fill the space to the right with the piece
        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize + piece_width, y as usize + v_block as usize);
            self.board[index] = piece_id;
        }
        // Clear the current position of the piece
        for v_block in 0..PIECE_DIMENSIONS[piece_id as usize].1 {
            let index = coordinate_to_index(x as usize, y as usize + v_block as usize);
            self.board[index] = BS;
        }
        self.piece_positions[piece_id as usize].0 += 1;
    }

    pub fn increase_cost(&mut self) {
        self.current_cost += 1;
    }

    pub fn print_board(&self) {
        for row in self.board.chunks(WIDTH) {
            for &cell in row {
                if cell == BS {
                    print!(" . "); // Represent blank space
                } else {
                    print!("{:02} ", cell); // Represent piece IDs
                }
            }
            println!(); // Move to the next row
        }
    }
}

impl AStarState<PuzzleKey> for Puzzle {
    fn key(&self) -> PuzzleKey {
        PuzzleKey::new(self)
    }

    fn h(&self) -> usize {
        let (x, y) = self.piece_positions[TARGET_PIECE as usize];
        let (target_x, target_y) = TARGET_POSITION;
        (x as isize - target_x as isize).abs() as usize + (y as isize - target_y as isize).abs() as usize
    }

    fn f(&self) -> usize {
        self.h() + self.g()
    }

    fn g(&self) -> usize {
        self.current_cost
    }

    fn is_goal(&self) -> bool {
        let (x, y) = self.piece_positions[TARGET_PIECE as usize];
        (x as usize, y as usize) == TARGET_POSITION
    }
}

impl TracedState<PuzzleKey, Movement> for Puzzle {
    fn generate_traced_successors(&self) -> Vec<(Self, Movement)> {
        let mut successors = Vec::new();

        // println!("Current state:");
        // self.print_board();

        for (piece_id, (_x, _y)) in self.piece_positions.iter().enumerate().map(|(i, &pos)| (i as u8, pos)) {
            // Try up
            if self.can_move_piece_up(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_up(piece_id);
                new_state.increase_cost();
                successors.push((new_state, Movement::Up(piece_id)));
            }

            // Try down
            if self.can_move_piece_down(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_down(piece_id);
                new_state.increase_cost();
                successors.push((new_state, Movement::Down(piece_id)));
            }

            // Try left
            if self.can_move_piece_left(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_left(piece_id);
                new_state.increase_cost();
                successors.push((new_state, Movement::Left(piece_id)));
            }

            // Try right
            if self.can_move_piece_right(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_right(piece_id);
                new_state.increase_cost();
                successors.push((new_state, Movement::Right(piece_id)));
            }
        }

        successors
    }
}

impl UntracedState<PuzzleKey> for Puzzle {
    fn generate_successors(&self) -> Vec<Self> {
        let mut successors = Vec::new();

        // println!("Current state:");
        // self.print_board();

        for (piece_id, (_x, _y)) in self.piece_positions.iter().enumerate().map(|(i, &pos)| (i as u8, pos)) {
            // Try up
            if self.can_move_piece_up(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_up(piece_id);
                new_state.increase_cost();
                successors.push(new_state);
            }

            // Try down
            if self.can_move_piece_down(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_down(piece_id);
                new_state.increase_cost();
                successors.push(new_state);
            }

            // Try left
            if self.can_move_piece_left(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_left(piece_id);
                new_state.increase_cost();
                successors.push(new_state);
            }

            // Try right
            if self.can_move_piece_right(piece_id) {
                let mut new_state = self.clone();
                new_state.move_piece_right(piece_id);
                new_state.increase_cost();
                successors.push(new_state);
            }
        }

        successors
    }
}



// Masive optimization: Two states are equal if if they look the same (if pieces with the same size are in the same position).
// It is not necessary that pieces are the same for the state to look the same.
#[derive(Clone)]
pub struct PuzzleKey {
    pub piece_dispositions: [((u8, u8), (u8, u8)); NUM_PIECES as usize] // (position, piece_size)
}

impl PuzzleKey {
    fn new(puzzle: &Puzzle) -> Self {
        let mut piece_dispositions = [((0, 0), (0, 0)); NUM_PIECES as usize];
        let mut checked_pieces = [false; NUM_PIECES as usize];

        let mut index = 0;
        for (piece_pos, &piece_id) in puzzle.board.iter().enumerate() {
            if piece_id == BS || checked_pieces[piece_id as usize] {
                continue;
            }

            let (x, y) = index_to_coordinate(piece_pos);
            let (width, height) = PIECE_DIMENSIONS[piece_id as usize];

            piece_dispositions[index] = ((x as u8, y as u8), (width as u8, height as u8));
            checked_pieces[piece_id as usize] = true;
            index += 1;
        }

        PuzzleKey {
            piece_dispositions
        }
    }
}

impl Debug for PuzzleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.piece_dispositions)
    }
}

impl PartialEq for PuzzleKey {
    fn eq(&self, other: &Self) -> bool {
        self.piece_dispositions == other.piece_dispositions
    }
}

impl Eq for PuzzleKey {}

impl Hash for PuzzleKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for &(pos, size) in &self.piece_dispositions {
            pos.hash(state);
            size.hash(state);
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Movement {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8)
}
