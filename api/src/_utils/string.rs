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

pub fn get_words<'a>(paragraph: &'a str) -> impl Iterator<Item = &'a str> {
  paragraph.split(|c: char| !c.is_alphanumeric())
}
