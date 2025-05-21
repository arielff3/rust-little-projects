use std::collections::HashSet;

/**
 * Leetcode 217 Contains Duplicate
 * Url: https://leetcode.com/problems/contains-duplicate/description/
 */

fn contains_duplicate(nums: Vec<i32>) -> bool {
  if nums.len() < 2 {
    return false
  }
  let mut map: HashSet<i32> = HashSet::new();
  for num in nums.iter() {
    if let Some(_) = map.get(num) {
      return true;
    }
    map.insert(*num);
  }
  false
}

pub fn run() {
  contains_duplicate(vec![1,2,3,1]);
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
      assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
      assert_eq!(contains_duplicate(vec![1,2,3,6]), false);
    }
}