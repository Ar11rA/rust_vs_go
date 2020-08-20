// https://leetcode.com/problems/search-suggestions-system

pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
  let mut results: Vec<Vec<String>> = vec![];
  products.sort();

  search_word
    .chars()
    .into_iter()
    .enumerate()
    .for_each(|(out_idx, _)| {
      results.push(
        products
          .clone()
          .into_iter()
          .filter(|item| {
            item.starts_with(&search_word
              .chars()
              .skip(0)
              .take(out_idx + 1)
              .collect::<String>())
          })
          .take(3)
          .collect(),
      );
    });
  return results;
}
