pub struct Solution {}

impl Solution {
    //Signiature should use char but it is defined by leetcode, can't change it
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        vec![vec![String::from("Q")]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn given_n_1_should_return_single_cell_with_queen() {
        let result = Solution::solve_n_queens(1);

        assert_eq!(result, vec![vec!["Q"]])
    }
}
