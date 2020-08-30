const PHONE_MAP: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

fn backtrack(acc: &mut Vec<String>, store: String, digits: String, ctr: i32) {
    if ctr == digits.len() as i32 {
        acc.push(store);
        return;
    }
    let map_index: usize = (digits
        .chars()
        .nth(ctr as usize)
        .unwrap()
        .to_digit(10)
        .unwrap() - 2
    ) as usize;
    let mapped: String = PHONE_MAP[map_index].to_string();
    mapped
        .chars()
        .into_iter()
        .for_each(|c| {
            backtrack(acc, format!("{}{}", store, c), digits.clone(), ctr + 1)
        });
}

pub fn get_combinations(digits: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    backtrack(&mut result, "".to_string(), digits, 0);
    return result;
}