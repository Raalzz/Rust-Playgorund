pub fn run() {
  //Print to console
  println!("Hello from the print.rs file");

  //Basic Formating
  println!("Number: {}", 1);
  println!("{} is {}", "Rust", "Awesome");

  //Positional Arguments
  println!("{1} is {0}", "Awesome", "Rust");

  //Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "Raalzz",
    activity = "Cricket"
  );

  //Placeholder traits
  println!("Binary: {:b} Hex: {:x} octal: {:o}", 10, 10, 10);

  //Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  //Basic math
  println!("10 + 10 = {}", 10 + 10);
}
