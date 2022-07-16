struct Solution {}

impl Solution {
    //Using a ``Vec<Vec<char>>` is bad but this is the signiature leetcode uses, don't get a choice
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_blank_board_is_valid_sudoku() {
        let input = vec![
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
            vec!['.','.','.','.','.','.','.','.','.',],
        ];

        assert_eq!(true, Solution::is_valid_sudoku(input));
    }
}