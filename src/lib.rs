pub fn find_matches(content: &String, pattern: &String, mut writer: impl std::io::Write) {
  for (index, line) in content.lines().enumerate() {
    if line.contains(pattern) {
      writeln!(writer, "{}: {line}", index + 1).unwrap();
    }
  }
}
