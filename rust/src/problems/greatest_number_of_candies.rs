// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
  let threshold: i32 = *candies.iter().max().unwrap();
  let candies_with_extra_ones: Vec<bool> = candies
    .iter()
    .map(|candy| {
      let final_sum: i32 = candy + extra_candies;
      return if final_sum >= threshold { true } else { false };
    })
    .collect();
  return candies_with_extra_ones;
}
