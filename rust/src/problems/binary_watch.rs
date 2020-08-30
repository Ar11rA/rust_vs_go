// https://leetcode.com/problems/binary-watch/ - Backtracking just the hours part
const HOUR_INDICES: [i32; 4] = [1, 2, 4, 8];

fn backtrack_hours(hours: &mut Vec<i32>, index: i32, result: i32, num: i32) {
    if result > 11 {
        return;
    }
    if num == 0 || index > 4 {
        hours.push(result);
        return;
    }
    for i in index..4 {
        backtrack_hours(hours, i + 1, result + HOUR_INDICES[i as usize], num - 1)
    }
}

pub fn get_hours(num: i32) -> Vec<i32> {
    let mut hours: Vec<i32> = Vec::new();
    for i in 0..=num {
        backtrack_hours(&mut hours, 0, 0, i);
    }
    return hours;
}
