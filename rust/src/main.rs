use crate::problems::apply_discount_every_n_orders::{Cashier, Employee};
use crate::problems::subrectangle_queries::Query;
use std::cell::RefCell;
use std::rc::Rc;

mod problems;

fn main() {
  let nums: Vec<i32> = vec![1, 2, 3, 4];
  println!(
    "Running sum problem: {:?}",
    problems::running_sum_of_array::running_sum(nums)
  );

  let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
  let extra_candies: i32 = 3;
  println!(
    "Extra candies problem: {:?}",
    problems::greatest_number_of_candies::kids_with_candies(candies, extra_candies)
  );

  let input_str: String = "codeleet".to_string();
  let str_indices: Vec<i32> = vec![4, 5, 6, 7, 0, 2, 1, 3];
  println!(
    "Shuffle string problem: {}",
    problems::shuffle_string::restore_string(input_str, str_indices)
  );

  let num: i32 = 234;
  println!(
    "Product minus sum problem: {}",
    problems::product_minus_sum::subtract_product_and_sum(num)
  );

  let tree = Some(Rc::new(RefCell::new(problems::bst_range_sum::TreeNode {
    val: 0,
    left: None,
    right: None,
  })));
  println!(
    "BST range sum problem: {}",
    problems::bst_range_sum::range_sum_bst(tree, 1, 10)
  );

  let start_time: Vec<i32> = vec![1, 2, 3];
  let end_time: Vec<i32> = vec![3, 2, 7];
  println!(
    "Homework at query time problem: {}",
    problems::homework_at_given_time::busy_student(start_time, end_time, 4)
  );

  let arr: Vec<i32> = vec![1, 2, 2, 1, 1, 3];
  println!(
    "Unique number of occurances: {}",
    problems::unique_number_of_occurances::unique_occurrences(arr)
  );

  let tiles: String = "AAB".to_string();
  println!(
    "Letter tile possibilities: {}",
    problems::letter_tile_possibilties::num_tile_possibilities(tiles)
  );

  let search_nums: Vec<i32> = vec![1, 3, 5, 6];
  let target: i32 = 0;
  println!(
    "Search insert position: {}",
    problems::search_insert_position::search_insert(search_nums, target)
  );

  println!(
    "Simple is subsequence: {}",
    problems::simple_is_subsequence::is_subsequence("abc".to_string(), "abcde".to_string())
  );

  let mut nums1: Vec<i32> = vec![1, 3, 5];
  let mut nums2: Vec<i32> = vec![1, 2, 4];
  println!(
    "Merge array problem {:?}",
    problems::merge_sorted_arrays::merge(&mut nums1, 3, &mut nums2, 3)
  );

  let nums3: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!(
    "Rotate Array problem {:?}",
    problems::rotate_array::rotate(nums3, 2)
  );

  let rectangle = vec![vec![1, 2], vec![3, 4]];
  let mut rect: problems::subrectangle_queries::SubrectangleQueries =
    problems::subrectangle_queries::Query::new(rectangle);
  println!("Subrectangle before {:?}", rect);
  rect.update_subrectangle(0, 0, 1, 1, 5);
  println!("Subrectangle after {:?}", rect);

  let strings: Vec<String> = vec![
    "abc".to_string(),
    "deq".to_string(),
    "mee".to_string(),
    "aqq".to_string(),
    "dkd".to_string(),
    "ccc".to_string(),
  ];
  println!(
    "Find and replace pattern problem {:?}",
    problems::find_and_replace_pattern::find_and_replace_pattern(strings, "bqq".to_string())
  );

  println!(
    "Single number problem 1 {}",
    problems::single_number::find(vec![0, 1, 0, 1, 0, 1, 99])
  );
  println!(
    "Single number problem 2 {:?}",
    problems::single_number::filter(vec![0, 1, 0, 1, 0, 1, 99])
  );

  let mut cashier: Cashier = problems::apply_discount_every_n_orders::Employee::new(
    3,
    50,
    vec![1, 2, 3, 4, 5, 6, 7],
    vec![100, 200, 300, 400, 300, 200, 100],
  );
  println!(
    "Cashier problem run 1 {}",
    cashier.get_bill(vec![1, 2], vec![1, 2])
  );
  println!(
    "Cashier problem run 2 {}",
    cashier.get_bill(vec![3, 7], vec![10, 10])
  );
  println!(
    "Cashier problem run 3 {}",
    cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1])
  );
  println!(
    "Cashier problem run 4 {}",
    cashier.get_bill(vec![4], vec![1])
  );

  println!(
    "Custom sort problem {}",
    problems::custom_sort_string::sort("cba".to_string(), "abcd".to_string())
  );

  println!(
    "Search suggestion system {:?}",
    problems::search_suggestions_system::suggested_products(
      vec![
        "mobile".to_string(),
        "mouse".to_string(),
        "moneypot".to_string(),
        "monitor".to_string(),
        "mousepad".to_string()
      ],
      "mouse".to_string(),
    )
  );

  println!("Remove invalid parenthesis {} {}",
     problems::remove_invalid_parenthesis::min_remove_to_make_valid(
       "lee(t(c)ode)".to_string()
     ),
     problems::remove_invalid_parenthesis::min_remove_to_make_valid(
       "())(()".to_string()
     )
  );

  println!("Binary watch problem {:?}", problems::binary_watch::get_hours(2));

  println!("Letter combinations {:?}",
     problems::letter_combinations::get_combinations("23".to_string())
  );
}
