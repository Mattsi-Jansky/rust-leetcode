use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    //Using a ``Vec<Vec<char>>` is bad but this is the signiature leetcode uses, don't get a choice
    pub fn is_valid_sudoku(board: &Vec<Vec<char>>) -> bool {
        let mut rows = HashMap::new();
        let mut columns = HashMap::new();
        let mut boxes = HashMap::new();

        for (y, row) in board.iter().enumerate() {
            let my_row = rows.entry(y).or_insert_with(HashSet::new);
            for (x, cell) in row.iter().enumerate() {
                let column = columns.entry(x).or_insert_with(HashSet::new);
                if cell != &'.' {
                    let box_key =  x / 3 + ((y / 3)  * 3);
                    let my_box = boxes.entry(box_key).or_insert_with(HashSet::new);

                    if !my_row.insert(cell)
                        || !column.insert(cell)
                        || !my_box.insert(cell) {
                        return false
                    }
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
