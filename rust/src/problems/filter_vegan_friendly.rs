// https://leetcode.com/problems/filter-restaurants-by-vegan-friendly-price-and-distance/

pub fn filter_restaurants(
  restaurants: Vec<Vec<i32>>,
  vegan_friendly: i32,
  max_price: i32,
  max_distance: i32) -> Vec<i32> {
  let mut filtered_restaurants: Vec<Vec<i32>> = restaurants
    .into_iter()
    .filter(|restaurant| {
      return if vegan_friendly == 1 {
        restaurant[2] == vegan_friendly
          && restaurant[3] <= max_price
          && restaurant[4] <= max_distance
      } else {
        restaurant[3] <= max_price
          && restaurant[4] <= max_distance
      }
    })
    .collect();
  filtered_restaurants
    .sort_by(|a, b| b[1].partial_cmp(&a[1]).unwrap());
  return filtered_restaurants
    .into_iter()
    .map(|restaurant| {
      restaurant[0]
    })
    .collect();
}