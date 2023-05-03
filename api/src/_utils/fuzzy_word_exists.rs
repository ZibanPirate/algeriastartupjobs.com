use rust_fuzzy_search::fuzzy_compare;

pub fn fuzzy_word_exists(word: &str, text: &str, threshold: f32) -> bool {
  // Split the text into words
  let words: Vec<&str> = text.split_whitespace().collect();

  // Compare each word in the text with the word to search for
  for word_in_text in words {
    let score = fuzzy_compare(word, word_in_text);
    // If the score is above the threshold, return true
    if score > threshold {
      return true;
    }
  }

  // If no word in the text is above the threshold, return false
  false
}
