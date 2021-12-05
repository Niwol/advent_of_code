use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let number_draws = data
        .split('\n')
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::new();
    let mut fields: BoardFields = [[0; 5]; 5];
    let (mut i, mut j) = (0, 0);

    data.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|n| {
            fields[i][j] = n;

            i += 1;
            if i >= 5 {
                i = 0;
                j += 1;
            }

            if j >= 5 {
                j = 0;
                boards.push(Board::new(fields));
            }
        });

    // V1
    let mut some_won = false;
    for &n in number_draws.iter() {
        for board in boards.iter_mut() {
            board.mark(n);

            if board.wins() {
                let score = board.calc_score(n);
                println!("Some board wins with a score of {}", score);

                some_won = true;
                break;
            }
        }

        if some_won {
            break;
        }
    }

    // V2
    for b in boards.iter_mut() {
        b.reset();
    }

    let mut number_iter = number_draws.iter();
    let mut last_draw = 0;
    while boards.len() > 1 {
        if let Some(&n) = number_iter.next() {
            last_draw = n;

            let mut i = 0;
            while i < boards.len() {
                boards[i].mark(n);

                if boards[i].wins() {
                    boards.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    while boards[0].wins() == false {
        if let Some(&n) = number_iter.next() {
            boards[0].mark(n);
            last_draw = n;
        }
    }

    let last_score = boards[0].calc_score(last_draw);
    println!("Last board to win has score {}", last_score);
}

type BoardFields = [[i32; 5]; 5];
type BoardMarkedFields = [[bool; 5]; 5];
struct Board {
    fields: BoardFields,
    marked_fields: BoardMarkedFields,
}

impl Board {
    fn new(fields: BoardFields) -> Board {
        Board {
            fields,
            marked_fields: [[false; 5]; 5],
        }
    }

    fn mark(&mut self, n: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.fields[i][j] == n {
                    self.marked_fields[i][j] = true;
                    return;
                }
            }
        }
    }

    fn wins(&self) -> bool {
        let mut win = true;

        // Collums
        for i in 0..5 {
            for j in 0..5 {
                if self.marked_fields[i][j] == false {
                    win = false
                }
            }
            if win {
                return true;
            }
            win = true;
        }

        win = true;
        // Rows
        for j in 0..5 {
            for i in 0..5 {
                if self.marked_fields[i][j] == false {
                    win = false
                }
            }
            if win {
                return true;
            }
            win = true;
        }

        return false;
    }

    fn calc_score(&self, last: i32) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.marked_fields[i][j] == false {
                    sum += self.fields[i][j];
                }
            }
        }

        sum * last
    }

    fn reset(&mut self) {
        for tab in &mut self.marked_fields {
            for b in tab {
                *b = false;
            }
        }
    }
}
