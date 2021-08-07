pub type Board = [[i32; 9]; 9];
pub type Space =  [[[i32;9];9];9];

pub struct Sudoku {
    pub board: Board,
    possibilities: Space,
    pub summary: Board
}
impl Sudoku {
    pub fn new() -> Self {
        Self{
            board: [[0;9];9],
            possibilities: [[[1;9];9];9],
            summary: [[9;9];9]
        }
    }
    pub fn from_string(self: &mut Self, input: &str)-> () {
        if input.chars().count() != 81 {
            panic!("Input string is not of length 81");
        }
        for (i,c) in input.chars().enumerate() {
            match c {
                '1'..='9' => {
                    let number = c as i32 - 48;
                    self.add_number(number, i%9, i/9);
                }
                '.' => {
                    continue
                }
                x => {
                    panic!("unable to understand input '{}'", x);
                }
            }
        }
    }
    pub fn check_sanity(self: &Self)-> bool {
        // Rule 1: sum across rows
        let mut rsum = Vec::with_capacity(9);
        for r in 0..9 {
            let mut sum = 0;
            for c in 0..9 {
                sum += self.board[c][r];
            }
            rsum.push(sum);
        }
        for sum in rsum {
            if sum != 45 {
                return false;
            }
        }
        let mut csum = Vec::with_capacity(9);
        for c in 0..9 {
            let mut sum = 0;
            for r in 0..9 {
                sum += self.board[c][r];
            }
            csum.push(sum);
        }
        for sum in csum {
            if sum != 45 {
                return false;
            }
        }
        for r in 0..3 {
            for c in 0..3 {
                let mut sum = 0;
                for row in r*3..(r*3)+3 {
                    for col in c*3..(c*3)+3 {
                        sum += self.board[col][row];
                    }
                }
                if sum != 45 {
                    return false;
                }
            }
        }
        true
    }
    fn add_number(self: &mut Self, number: i32, col: usize, row:usize)-> () {
        self.board[col][row] = number;
        let n = number -1;
        for p in 0..9 {
            self.possibilities[p][col][row] = 0;
        }
        for c in 0..9 {
            self.possibilities[n as usize][c][row] = 0;
        }
        for r in 0..9 {
            self.possibilities[n as usize][col][r] = 0;
        }
        for r in (row/3)*3 .. ((row/3)+1)*3 {
            for c in (col/3)*3 .. ((col/3)+1)*3 {
                self.possibilities[n as usize][c][r] = 0;
            }
        }
        self.possibilities[n as usize][col][row] = 1;
        self.update_summary();
    }
    fn update_summary(self: &mut Self)-> () {
        for row in 0..9 {
            for col in 0..9 {
                let mut sum: i32 = 0;
                for p in 0..9 {
                    sum += self.possibilities[p][col][row];
                }
                self.summary[col][row] = sum;
            }
        }
    }
    fn next_move(self: &Self)-> Option<(i32, usize, usize)> {
        let mut result = self.obvious();
        if result.is_none() {
            result = self.conclude_row();
        }
        if result.is_none() {
            result = self.conclude_col();
        }
        if result.is_none() {
            result = self.conclude_submatrix();
        }
        result
    }
    fn conclude_col(self: &Self)-> Option<(i32,usize,usize)> {
        for c in 0..9 {
            for p in 0..9 {
                let mut sum = 0;
                let mut row = 9;
                for r in 0..9 {
                    sum += self.possibilities[p][c][r];
                    if self.possibilities[p][c][r] == 1 {
                        row = r;
                    }
                }
                if sum == 1 && self.summary[c][row]!=1 {
                    return Some(((p+1) as i32, c, row));
                }
            }
        }
        None
    }
    fn conclude_row(self: &Self)-> Option<(i32,usize,usize)> {
        for r in 0..9 {
            for p in 0..9 {
                let mut sum = 0;
                let mut col = 9;
                for c in 0..9 {
                    sum += self.possibilities[p][c][r];
                    if self.possibilities[p][c][r] == 1 {
                        col = c;
                    }
                }
                if sum == 1 && self.summary[col][r]!=1 {
                    return Some(((p+1) as i32, col, r));
                }
            }
        }
        None
    }
    fn conclude_submatrix(self: &Self)-> Option<(i32, usize, usize)> {
        for r in 0..3 {
            for c in 0..3 {
                for p in 0..9 {
                    let mut sum = 0;
                    let mut rrow = 9;
                    let mut ccol = 9;
                    for row in r*3..(r*3)+3 {
                        for col in c*3..(c*3)+3 {
                            sum += self.possibilities[p][col][row];
                            if self.possibilities[p][col][row] == 1 {
                                rrow = row;
                                ccol = col;
                            }
                        }
                    }
                    if sum == 1 && self.summary[ccol][rrow]!=1 {
                       return Some(((p+1) as i32, ccol, rrow));
                    } 
                }
            }
        }
        None
    }
    fn obvious(self: &Self)-> Option<(i32, usize, usize)> {
        let mut number: i32 = 0;
        let mut r: usize = 9;
        let mut c: usize = 9;
        'outer: for row in 0..9 {
            for col in 0..9 {
                if self.summary[col][row] == 1 && self.board[col][row] == 0 {
                    r = row; c = col;
                    break 'outer;
                }
            }
        }
        if r < 9 {
            for p in 0..9 {
                if self.possibilities[p][c][r] == 1 {
                    number = p as i32;
                    break;
                }
            }
        }
        if r < 9 {Some((number+1, c, r))}
        else {None}
    }
    pub fn solve(self: &mut Self)->() {
        while let Some((n, c, r)) = self.next_move() {
            self.add_number(n, c, r);
        }
    }
}
