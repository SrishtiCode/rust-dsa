use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut seen = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let needed = target - num;

        if let Some(&j) = seen.get(&needed) {
            return vec![j, i];
        }

        seen.insert(num, i);
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(nums, target);

    println!("{:?}", result);
}