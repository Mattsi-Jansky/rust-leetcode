pub struct Solution {}

impl Solution {
    // Signiature should use char but it is defined by leetcode, can't change it.
    // Also, `n` should probably be `usize`.
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = vec![];

        let mut frontline = vec![];

        for i in 0..n {
            let mut board = vec![None; 9];
            board[0] = Some(i);
            frontline.push(board);
        }

        while !frontline.is_empty() {
            let board = frontline.pop().unwrap();
            if incomplete(&board) {
                add_next_possibilities(board, &mut frontline);
            }
            else { result.push(vec![String::from("Solution")]) }
        }

        result
    }
}

fn incomplete(board: &[Option<usize>]) -> bool {
    board.iter().any(|&b| matches!(b, None))
}

fn add_next_possibilities(board: Vec<Option<usize>>, frontline: &mut Vec<Vec<Option<usize>>>) {
    for i in 0..board.len() {
        if board[i].is_some() { continue; }
        else {
            for j in 0..board.len() {
                let mut new_board = board.clone();
                new_board[i] = Some(j);
                if is_valid(&new_board) {
                    frontline.push(new_board);
                }
            }
        }
    }
}

fn is_valid(board: &[Option<usize>]) -> bool {
    let mut result = true;

    for x in 0..board.len() {
        if x == board.len()-1 || matches!(board[x+1], None) {
            let y = board[x].unwrap();
            if board.iter().filter(|yy| yy == &&Some(y)).count() > 1 {
                result = false;
            }

            let mut diagonals = vec![];
            for i in 1..board.len() {
                if x >= i && y >= i { diagonals.push((x-i,y-i)) }
                if x < board.len() - i && y < board.len() - i { diagonals.push((x+i,y+i)) }
                if x < board.len() - i && y >= i { diagonals.push((x+i,y-i)) }
                if x >= i && y < board.len() - i { diagonals.push((x-i,y+i)) }
            }

            for (x,y) in diagonals {
                if board[x] == Some(y) {
                    result = false;
                }
            }

            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]
    fn given_n_1_should_return_single_cell_with_queen() {
        let result = Solution::solve_n_queens(1);

        assert_eq!(result, vec![vec!["Q"]])
    }

    #[test]
    #[ignore]
    fn given_n_4() {
        let result = Solution::solve_n_queens(4);

        assert_eq!(result, vec![
            [".Q..","...Q","Q...","..Q."],
            ["..Q.","Q...","...Q",".Q.."]
        ])
    }

    #[test]
    fn incomplete_returns_true_given_board_with_none() {
        let mut board = vec![None; 4];
        board[0] = Some(0);

        let result = incomplete(&board);

        assert_eq!(true, result);
    }

    #[test]
    fn incomplete_returns_false_given_board_with_no_none() {
        let board = vec![Some(1); 4];

        let result = incomplete(&board);

        assert_eq!(false, result);
    }

    #[test]
    #[ignore]
    fn add_next_possibilities_adds_next_possible_steps() {
        let mut board = vec![None; 4];
        board[0] = Some(0);
        let mut frontline = vec![];

        add_next_possibilities(board, &mut frontline);

        assert_eq!(frontline, vec![
            vec![Some(0),Some(2),None,None],
            vec![Some(0),Some(3),None,None]
        ])
    }

    #[test]
    fn given_same_y_value_is_valid_returns_false() {
        let mut board = vec![None; 4];
        board[0] = Some(0);
        board[1] = Some(0);

        let result = is_valid(&board);

        assert_eq!(false, result)
    }

    #[test]
    fn given_diagonal_collision_is_valid_returns_false() {
        let mut board = vec![None; 4];
        board[0] = Some(0);
        board[1] = Some(1);

        let result = is_valid(&board);

        assert_eq!(false, result)
    }
}
