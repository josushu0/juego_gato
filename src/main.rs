fn main() {
    let board = ['x'; 9];
    println!("-----Juego de Gato-----");
    draw_board(&board);
}

fn draw_board(board: &[char; 9]) {
    for i in 0..2 {
        print!("{} | ", board[i]);
    }
    println!("{} ", board[2]);
    println!("---------");
    for i in 3..5 {
        print!("{} | ", board[i]);
    }
    println!("{} ", board[5]);
    println!("---------");
    for i in 6..8 {
        print!("{} | ", board[i]);
    }
    println!("{} ", board[8]);
}

