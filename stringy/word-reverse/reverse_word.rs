#!/usr/bin/env runner

fn reverse_words(string: &str) -> String {
    string.to_owned()
    .split(" ")
    .map(|x| x.chars().rev().collect())
    .collect::<Vec<String>>()
    .join(" ")
}




  assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god".to_string());
  assert_eq!(reverse_words("apple"), "elppa".to_string());
  assert_eq!(reverse_words("a b c d"),"a b c d".to_string());
  assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow".to_string());
