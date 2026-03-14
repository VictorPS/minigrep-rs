pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
