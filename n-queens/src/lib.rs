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
            else if is_valid(board) { result.push(vec![String::from("Solution")]) }
        }

        result
    }
}

fn incomplete(board: &Vec<Option<usize>>) -> bool {
    board.iter().any(|&b| matches!(b, None))
}

fn add_next_possibilities(board: Vec<Option<usize>>, frontline: &mut Vec<Vec<Option<usize>>>) {
    todo!()
}

fn is_valid(board: Vec<Option<usize>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn given_n_1_should_return_single_cell_with_queen() {
    //     let result = Solution::solve_n_queens(1);
    //
    //     assert_eq!(result, vec![vec!["Q"]])
    // }
    //
    // #[test]
    // fn given_n_4() {
    //     let result = Solution::solve_n_queens(4);
    //
    //     assert_eq!(result, vec![
    //         [".Q..","...Q","Q...","..Q."],
    //         ["..Q.","Q...","...Q",".Q.."]
    //     ])
    // }

    #[test]
    fn incomplete_returns_true_given_board_with_none() {
        let mut board = vec![None; 4];
        board[0] = Some(0);

        let result = incomplete(&board);

        assert_eq!(true, result);
    }

    #[test]
    fn incomplete_returns_false_given_board_with_no_none() {
        let mut board = vec![Some(1); 4];

        let result = incomplete(&board);

        assert_eq!(false, result);
    }
}
