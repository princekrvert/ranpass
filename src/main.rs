use rand::Rng;
use std::io::Write;
//Author Prince Kumar 
//date 9 jun 2023
//make a function to write string to the file
fn writefile(pass: String){
    let mut file = std::fs::File::create("data.txt").expect("create failed");
   file.write_all(pass.as_bytes()).expect("write failed");
}
fn main() {
    let mut rng = rand::thread_rng();
    let rand_num:u128 = rng.gen_range(0..100);
    writefile(rand_num.to_string());
    println!("Hello, world!");
}
