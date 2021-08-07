use rudoku::rudoku;

fn main() {
    let mut sudoku = rudoku::Sudoku::new();
    // Solveable ones:
    // sudoku.from_string("1.64....7..9..7.....892..46.6.1.42...81....3.2..8.56.1..........34.6.7...17...96.");
    // sudoku.from_string("...84...5...1...36.57.26.1.......3...6.71....81...5...1..56.7.....27...9.94......");
    // sudoku.from_string("...2.....8.9...1...2.......6.3..9....7.6...5.....4.9.3.4..8..3...69...7..95..1..2");
    sudoku.from_string("...24...1.......6.36....574..3.8..1.5.4.....8...7........6.9.....8...6...7...4.92");
    
    // Cannot solve this:
    // sudoku.from_string("..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..");
    
    sudoku.solve();
    println!("============ Board ============");
    print_board(&sudoku.board);
    println!("=========== Summary ===========");
    print_board(&sudoku.summary);
    println!("correctness: {}", sudoku.check_sanity());
}

fn print_board(board: &[[i32; 9]; 9])-> () {
    println!("+---------+---------+---------+");
    for row in 0..9 {
        print!("|");
        for col in 0..9 {
            match board[col][row] {
                0 => {
                    print!(" . ");
                }
                _ => {
                    print!("{:^3}", board[col][row]);
                }
            }
            if (col+1)%3 == 0 {
                print!("|");
            }
        }
        if (row+1)%3 == 0 {
            print!("\n+---------+---------+---------+");
        }
        println!("");
    }
}