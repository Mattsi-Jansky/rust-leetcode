pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = 0;
    let mut total = 0;
    
    loop {
        let new_total = sum(&nums, start, end);
        if new_total > total { total = new_total; }
        end += 1;
        if end == nums.len() { break; }
    }
    
    total
}

fn sum(numbs: &Vec<i32>, start: usize, end: usize) -> i32 {
    let mut total = 0;

    for i in start..=end {
        total += numbs[i];
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
    fn does_the_thing() {
        let input = vec![-2,1,-3,4,-1,2,1,-5,4];

        let result = max_sub_array(input);

        assert_eq!(6, result);
    }
}