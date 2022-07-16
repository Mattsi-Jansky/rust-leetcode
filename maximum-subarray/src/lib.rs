pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut end = 0;
    let mut total = nums[0];
    let mut running_total = 0;
    
    while end < nums.len() && nums.len() != 1 {
        running_total += nums[end];
        if running_total > total { total = running_total; }
        if running_total < 0 {
            end += 1;
            running_total = 0;
            continue;
        }
        end += 1;
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_total() {
        let input = vec![1,1];

        let result = max_sub_array(input);

        assert_eq!(2, result);
    }

    #[test]
    fn calculates_total_ignoring_last_negative() {
        let input = vec![1,1,-1];

        let result = max_sub_array(input);

        assert_eq!(2, result);
    }

    #[test]
    fn calculates_total_ignoring_first_negative() {
        let input = vec![-1,1,1,-1];

        let result = max_sub_array(input);

        assert_eq!(2, result);
    }

    #[test]
    fn single_negative_number() {
        let input = vec![-1];

        let result = max_sub_array(input);

        assert_eq!(-1, result);
    }

    #[test]
    fn two_one_of_which_is_negative() {
        let input = vec![-2,1];

        let result = max_sub_array(input);

        assert_eq!(1, result);
    }

    #[test]
    fn two_negative() {
        let input = vec![-2,-1];

        let result = max_sub_array(input);

        assert_eq!(-1, result);
    }
    
    #[test]
    fn does_the_thing() {
        let input = vec![-2,1,-3,4,-1,2,1,-5,4];

        let result = max_sub_array(input);

        assert_eq!(6, result);
    }
}