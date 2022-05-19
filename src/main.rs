use crate::user_input::*;
use rand::Rng;
use ascii_converter::*;

mod user_input;

fn main() {
    let mut rng = rand::thread_rng();  
    let mut characters: i32 = get_chars();
    let mut output = String::new();
    let mut temp = Vec::new();
  
    for n in 0..get_iterators(){
        for n in 0..characters
        {
            temp.push(rng.gen_range(33..126));

        }

        match decimals_to_string(&temp){
            Ok(num) => println!("{}\n\n", num),
            Err(e) => println!("There was an Error:\n{}", e),    
        };

        temp.clear();
        //println!("The generated output is: {}", output);
        
        
    }
}
