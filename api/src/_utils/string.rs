pub fn slugify(s: &String) -> String {
  let filtered = s
    .chars()
    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
    .collect::<String>();
  let mut slug = slug::slugify(filtered)
    .chars()
    .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
    .collect::<String>();
  slug.truncate(50);
  slug
}

pub fn escape_single_quote(s: &String) -> String {
  s.replace("'", "\\'")
}

pub fn escape_double_quote(s: &String) -> String {
  s.replace("\"", "\\\"")
}

pub fn escape_new_line(s: &String) -> String {
  s.replace("\n", "\\n")
}

pub fn escape_new_line_with_br(s: &String) -> String {
  s.replace("\n", "<br>")
}

// @TODO-ZM: change this to get_searchable_words
pub fn get_words<'a>(paragraph: &'a str) -> impl Iterator<Item = &'a str> {
  paragraph.split(|c: char| !c.is_alphanumeric())
}

// @TODO-ZM: filter out common words
pub fn get_searchable_words(paragraph: &String) -> Vec<String> {
  paragraph
    .split(|c: char| !c.is_alphanumeric())
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}
