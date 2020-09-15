package problems

// https://leetcode.com/problems/implement-trie-prefix-tree
type Trie struct {
  isEnd bool
  children []*Trie
}

/** Initialize your data structure here. */
func NewTrie() *Trie {
  return &Trie{
      isEnd: false,
      children: make([]*Trie, 26),
  }
}

/** Inserts a word into the trie. */
func (t *Trie) Insert(word string)  {
  curr := t
  for i := 0; i < len(word); i++ {
    c := int(word[i] - 'a')
    if curr.children[c] == nil {
      curr.children[c] = NewTrie()
    }
    curr = curr.children[c]
  }
  curr.isEnd = true
}

/** Returns if the word is in the trie. */
func (t *Trie) Search(word string) bool {
  cur := t
  for i := 0; i < len(word); i++ {
    c := int(word[i] - 'a')
    if cur.children[c] == nil {
      return false
    }
    cur = cur.children[c]
  }

  return cur.isEnd
}

/** Returns if there is any word in the trie that starts with the given prefix. */
func (t *Trie) StartsWith(prefix string) bool {
  cur := t
  for i := 0; i < len(prefix); i++ {
    c := int(prefix[i] - 'a')
    if cur.children[c] == nil {
      return false
    }
    cur = cur.children[c]
  }

  return true
}

