use std::thread;
use std::time::Duration;

static TERM_WIN_SIZE: i32 = 20; /* width = height */
static MILLI_SEC_PER_1_GEN: u64 = 100;
static MAX_GEN: u16 = 1000;

type Pos = (i32, i32);

fn main() {
    run();
}

fn run() {
    // glider
    let mut board: Vec<Pos> = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

    for _ in 0..MAX_GEN {
        print_board(&board);
        board = next_gen(&board);
        thread::sleep(Duration::from_millis(MILLI_SEC_PER_1_GEN));
    }
}

fn print_board(board: &Vec<Pos>) {
    clear_screen();
    for (row, col) in board {
        cursor_position((*row, *col));
        print_cell();
        println!("\n");
    }
}

fn clear_screen() {
    print!("\x1b[2J");
}

fn cursor_position((row, col): Pos) {
    let col = col * 2;
    print!("\x1b[{};{}H", row + 1, col + 1);
}

fn print_cell() {
    print!("\x1b[46m  \x1b[0m");
}

fn is_dead((row, col): Pos, board: &Vec<Pos>) -> bool {
    !is_alive((row, col), board)
}

fn is_alive((row, col): Pos, board: &Vec<Pos>) -> bool {
    board.contains(&(row, col))
}

fn arounds((row, col): Pos) -> Vec<Pos> {
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .iter()
    .map(|&(r, c)| (div_inv(r, TERM_WIN_SIZE - 1), div_inv(c, TERM_WIN_SIZE - 1)))
    .collect()
}

fn div_inv(n: i32, m: i32) -> i32 {
    if n >= 0 {
        n % m
    } else {
        (n + m) % m
    }
}

fn next_gen(board: &Vec<Pos>) -> Vec<Pos> {
    let mut next_board: Vec<Pos> = vec![];
    for row in 0..TERM_WIN_SIZE - 1 {
        for col in 0..TERM_WIN_SIZE - 1 {
            let live_cell_count = live_cell_count((row, col), board);
            if is_dead((row, col), board) {
                if live_cell_count == 3 {
                    next_board.push((row, col));
                }
            } else if is_alive((row, col), board) {
                if live_cell_count == 2 || live_cell_count == 3 {
                    next_board.push((row, col));
                } else if live_cell_count <= 1 || 4 <= live_cell_count {
                    continue;
                }
            }
        }
    }
    next_board
}

fn live_cell_count((row, col): Pos, board: &Vec<Pos>) -> u8 {
    arounds((row, col))
        .iter()
        .cloned()
        .filter(|&p| is_alive(p, board))
        .count() as u8
}