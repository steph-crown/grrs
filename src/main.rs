use std::env::args;

fn main() {
  let pattern = get_arg(1, "No pattern is given");
  let path = get_arg(2, "No path is given");

  println!("pattern {:?}, path {:?}", pattern, path);
}

fn get_arg(pos: usize, err_message: &'static str) -> String {
  let value = args().nth(pos).expect(err_message);

  value
}
