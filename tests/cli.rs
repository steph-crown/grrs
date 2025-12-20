#[test]
fn test_find_matches() {
  let content = String::from("lorem ipsum\ndolor sit amet");
  let pattern = String::from("lorem");

  let mut result = Vec::new();
  grrs::find_matches(&content, &pattern, &mut result);
  println!("{:?}", result);
  assert_eq!(result, b"1: lorem ipsum\n");
}
