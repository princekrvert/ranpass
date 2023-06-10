use rand::Rng;
use std::io::Write;
use std::env::args;
//Author Prince Kumar
//date 9 jun 2023
//make a function to write string to the file
fn writefile(pass: String){
    let mut file = std::fs::File::create("data.txt").expect("create failed");
   file.write_all(pass.as_bytes()).expect("write failed");
}
fn main() {
    // create an empty vector to keep track of the pass
    let mut passlist: Vec<u128> = Vec::new();
    // Now collect the args into a vector
    let arguments:Vec<String> = args().collect();
    let mut rng = rand::thread_rng();
    for i in 1..=arguments[3].parse().unwrap(){
    let rand_num:u128 = rng.gen_range(arguments[1].parse().unwrap()..arguments[2].parse().unwrap());
    if i == 1
    {continue;}else{
                        passlist.push(rand_num);
                            for pass in passlist.iter(){
                                if *pass == rand_num {
                                continue;}else{
                                    println!("{} {}",rand_num,pass);
                                    writefile(rand_num.to_string());
                                }

        }
    }}

}
