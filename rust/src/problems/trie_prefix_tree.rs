#[derive(Default)]
struct Trie {
  is_end: bool,
  children: [Option<Box<Trie>>; 26],
}

pub trait TrieOps {
  fn new() -> Self;
  fn insert(&mut self, word: String);
  fn search(&self, word: String) -> bool;
  fn starts_with(&self, prefix: String) -> bool;
}

impl TrieOps for Trie {
  fn new() -> Self {
    return Default::default();
  }

  fn insert(&mut self, word: String) {
    let mut curr = self;
    for c in word.chars() {
      let val = &curr.children[c as usize - 'a' as usize];
      match val {
        None => curr.children[c as usize - 'a' as usize] = Some(Box::new(Trie::new())),
        Some(_) => ()
      }
      curr = curr.children[c as usize - 'a' as usize].as_mut().unwrap();
    }
    curr.is_end = true;
  }

  fn search(&self, word: String) -> bool {
    let mut cur = self;
    for c in word.chars()
    {
      match cur.children[c as usize - 'a' as usize].as_ref()
      {
        None => return false,
        Some(n) => cur = n.as_ref(),
      }
    }
    return cur.is_end;
  }

  fn starts_with(&self, prefix: String) -> bool {
    let mut cur = self;
    for c in prefix.chars()
    {
      match cur.children[c as usize - 'a' as usize].as_ref()
      {
        None => return false,
        Some(n) => cur = n.as_ref(),
      }
    }
    return true;
  }
}