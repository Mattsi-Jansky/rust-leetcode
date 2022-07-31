use std::collections::{HashMap, HashSet};

struct Solution {}

const ROW_LEN: u32 = 9;

impl Solution {
    //Using a ``Vec<Vec<char>>` is bad but this is the signiature leetcode uses, don't get a choice
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = HashMap::new();
        let mut columns = HashMap::new();
        let mut result = true;

        for (y, row) in board.iter().enumerate() {
            rows.insert(y, HashSet::new());
            for (x, cell) in row.iter().enumerate() {
                if !columns.contains_key(&x) { columns.insert(x, HashSet::new()); }
                if cell != &'.' && !rows.get_mut(&y).unwrap().insert(cell) {
                    result = false;
                }
                if cell != &'.' && !columns.get_mut(&x).unwrap().insert(cell) {
                    result = false;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_blank_board_is_valid_sudoku() {
        let input = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(true, Solution::is_valid_sudoku(input));
    }

    #[test]
    fn given_valid_board_returns_true() {
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(true, Solution::is_valid_sudoku(input));
    }

    #[test]
    fn given_duplicate_numbers_in_row_returns_false() {
        let input = vec![
            vec!['3', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(false, Solution::is_valid_sudoku(input));
    }

    #[test]
    fn given_duplicate_numbers_in_column_returns_false() {
        let input = vec![
            vec!['.', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '3', '.', '.', '.', '.', '.', '7', '9'],
        ];

        assert_eq!(false, Solution::is_valid_sudoku(input));
    }
}
