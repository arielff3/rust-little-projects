use std::collections::HashMap;

/**
 * Leetcode: 1 Two Sum
 * Url: https://leetcode.com/problems/two-sum/description/
 */

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
      let c = target - num;

      if let Some(&index) = map.get(&c) {
        return vec![index as i32, i as i32];
      }
      map.insert(*num, i);
    }

    vec![]
}

pub fn run() {
    println!("Two Pointer");
    let result = two_sum(vec![2, 7, 11, 15], 9);
    println!("Result: {:?}", result);
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }
}
