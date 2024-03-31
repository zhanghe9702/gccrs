use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lkmap = HashMap::<i32, i32>::new();
        for (i, &value) in nums.iter().enumerate() {
            lkmap.insert(value, i as i32);
        }
        for (i, &value) in nums.iter().enumerate(){
            if let Some(&j) = lkmap.get(&(target -value)) {
                if j != i as i32 {
                    return vec![i as i32, j];
                }
            }
        }
        panic!();
}

fn main() {
	assert_eq!(two_sum(vec![2,7,11,15],9), vec![0,1]);
}