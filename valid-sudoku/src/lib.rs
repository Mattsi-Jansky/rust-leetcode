use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    //Using a ``Vec<Vec<char>>` is bad but this is the signiature leetcode uses, don't get a choice
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0; 9];
        let mut columns = [0; 9];
        let mut boxes = [0; 9];

        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell != &'.' {
                    let bit = 1 << cell.to_digit(10).unwrap();
                    let box_key =  x / 3 + ((y / 3)  * 3);

                    if rows[y] & bit != 0
                        || columns[x] & bit != 0
                        || boxes[box_key] & bit != 0 {
                        return false
                    }

                    rows[y] |= bit;
                    columns[x] |= bit;
                    boxes[box_key] |= bit;
                }
            }
        }

        true
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

        assert_eq!(true, Solution::is_valid_sudoku(&input));
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

        assert_eq!(true, Solution::is_valid_sudoku(&input));
    }

    #[test]
    fn given_complete_valid_board_returns_true() {
        let input = vec![
            vec!['8', '2', '7', '1', '5', '4', '3', '9', '6'],
            vec!['9', '6', '5', '3', '2', '7', '1', '4', '8'],
            vec!['3', '4', '1', '6', '8', '9', '7', '5', '2'],
            vec!['5', '9', '3', '4', '6', '8', '2', '7', '1'],
            vec!['4', '7', '2', '5', '1', '3', '6', '8', '9'],
            vec!['6', '1', '8', '9', '7', '2', '4', '3', '5'],
            vec!['7', '8', '6', '2', '3', '5', '9', '1', '4'],
            vec!['1', '5', '4', '7', '9', '6', '8', '2', '3'],
            vec!['2', '3', '9', '8', '4', '1', '5', '6', '7'],
        ];

        assert_eq!(true, Solution::is_valid_sudoku(&input));
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

        assert_eq!(false, Solution::is_valid_sudoku(&input));
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

        assert_eq!(false, Solution::is_valid_sudoku(&input));
    }

    #[test]
    fn given_duplicate_numbers_in_box_returns_false() {
        let input = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '2'],
        ];

        assert_eq!(false, Solution::is_valid_sudoku(&input));
    }
}
