use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
// use unicase::UniCase;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    /*
        everything to lowercase first
        then split to graphemes and sort
        collect back to string
        do the same for each of possible anagrams
        then check for eq
        don't forget to eliminate the case where it just gives you -
        the same string but you return true
    */

    let mut w: Vec<&str> = word.to_lowercase().graphemes(true).collect();
    w.sort_unstable();

    // let word_lowercase = word.to_lowercase();
    // let mut word_vec: Vec<&str> = word_lowercase.graphemes(true).collect();
    // word_vec.sort_unstable();
    // let word_sorted: String = word_vec.into_iter().collect();

    // for thing in possible_anagrams {
    //     let thing_lowercase = thing.to_lowercase();
    //     let mut thing_vec: Vec<&str> = thing_lowercase.graphemes(true).collect();

    //     thing_vec.sort_unstable();

    //     let thing_sorted: String = thing_vec.into_iter().collect();

    //     let is_eq = thing_sorted.eq(&word_sorted);
    //     let is_same_str = thing_lowercase.eq(&word_lowercase);

    //     if is_eq && !is_same_str {
    //         anagrams.insert(*thing);
    //     }
    // }
    anagrams
}
