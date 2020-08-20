pub fn num_tile_possibilities(tiles: String) -> i32 {
  let mut dict: Vec<i32> = vec![0; 26];
  for c in tiles.chars() {
    dict[c as usize - 'A' as usize] += 1;
  }
  let mut count: i32 = 0;
  dfs(&mut dict, &mut count);
  return count;
}

fn dfs(dict: &mut Vec<i32>, count: &mut i32) {
  for i in 0..dict.len() {
    if dict[i] > 0 {
      *count += 1;
      dict[i] -= 1;
      dfs(dict, count);
      dict[i] += 1;
    }
  }
}
