use std::io::Read;



fn main() {
  
  let mut proc = std::process::Command::new("perl").args(["./src/gen.pl"]).output().unwrap();
  println!("cargo:rerun-if-changed=./src/lib.rs");
  let _ = std::fs::remove_file("./src/src.txt");
  //std::fs::remove_file("./src/lib.rs").unwrap();
  std::fs::write("./src/src.txt", proc.stdout).unwrap();
}