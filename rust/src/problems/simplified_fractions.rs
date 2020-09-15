pub fn simplified_fractions(n: i32) -> Vec<String> {
  let mut results: Vec<String> = Vec::new();
  if n == 1 {
    return results;
  }
  for i in 1..=n {
    for j in i + 1..=n {
      println!("{}, {}, {}", i, j, gcd(j, i));
      if gcd(j, i) == 1 {
        let fraction: String = format!("{}/{}", i, j);
        results.push(fraction);
      }
    }
  }
  return results;
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
  while b != 0 {
    a = b;
    b = a % b;
  }
  return a;
}