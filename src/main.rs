extern crate heliometer;

use heliometer::*;

fn main() {
  let input = "+.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";


  let output = execute(&input, std::io::stdin(), std::io::stdout());

  // println!("{}", output.unwrap())
}
