mod problems;


fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    println!("Running sum problem: {:?}", problems::running_sum_of_array::running_sum(nums));

    let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
    let extra_candies: i32 = 3;
    println!("Extra candies problem: {:?}", problems::greatest_number_of_candies::kids_with_candies(candies, extra_candies));
}
