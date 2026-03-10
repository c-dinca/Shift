use shift_detect;

fn main() {
   match shift_detect::detect(String::from("./package.json"))
   {
      Ok(stack) => {
          println!("Detected stack:");

          match stack.node_version {
              Some(version) => println!("Node version: {}", version),
              None => println!("Node version: not specified"),
          }
          println!("Packages: {}", stack.packages.join(","));
      }
       Err(e) => {
           println!("{}", e);
       }
   }
}
