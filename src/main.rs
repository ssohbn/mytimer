use std::time::SystemTime;
use std::process::Command;
use std::env::{self, Args};

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 { panic!("need more arg fatty")}
  let start = SystemTime::now();
  
  let mut output = Command::new(&args[1]);
  for arg in &args
  {  
   output.arg(arg); 
  }
  output.output().unwrap();
  
  let elapsed = start.elapsed().unwrap();
  println!("{}", elapsed.as_secs_f64());
}