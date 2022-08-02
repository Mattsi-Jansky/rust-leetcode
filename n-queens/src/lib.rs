pub struct Solution {}

impl Solution {
    // Signiature should use char but it is defined by leetcode, can't change it.
    // Also, `n` should probably be `usize`.
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = vec![];

        let mut frontline = vec![];

        for i in 0..n {
            let mut board = vec![None; n];
            board[0] = Some(i);
            frontline.push(board);
        }

        while !frontline.is_empty() {
            let board = frontline.pop().unwrap();
            if incomplete(&board) {
                add_next_possibilities(board, &mut frontline);
            }
            else {
                let valid_result = serialise_board(board, n);
                result.push(valid_result)
            }
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
            break;
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

fn serialise_board(board: Vec<Option<usize>>, n: usize) -> Vec<String> {
    let mut valid_result = vec![];
    for _ in 0..n {
        valid_result.push((0..board.len()).fold(String::new(), |s,_| s + "."));
    }

    for (y, x) in board.iter().map(|x| x.unwrap()).enumerate() {
        valid_result.get_mut(x).unwrap()
            .replace_range(y..y+1, "Q");
    }

    valid_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_n_1_should_return_single_cell_with_queen() {
        let result = Solution::solve_n_queens(1);

        assert_eq!(result, vec![vec!["Q"]])
    }

    #[test]
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

    #[test]
    fn should_serialise_board() {
        let mut board = vec![None; 4];
        board[0] = Some(2);
        board[1] = Some(0);
        board[2] = Some(3);
        board[3] = Some(1);

        let result = serialise_board(board, 4);

        assert_eq!(vec![
            ".Q..",
            "...Q",
            "Q...",
            "..Q."
        ], result);
    }
}
