use std::io;

fn main() {
    let board = [' '; 9];
    println!("-----Juego de Gato-----");
    println!("Jugar primer turno? s/n");
    loop {
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer == "s" {
            next_turn(board);
            break;
        }
        else if answer == "n" {
            ia_turn(&board);
            next_turn(board);
            break;
        }
        else {
            println!("Escriba una opción valida");
        }
    }
}

fn next_turn(board: [char; 9]) {

}

fn ia_turn(board: &[char; 9]) {

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

