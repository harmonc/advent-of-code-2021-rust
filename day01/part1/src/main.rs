use std::fs;
use std::str;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("in.txt")
        .expect("Something went wrong reading the file");
    

    let vec: Vec<i32> = contents.split("\r\n").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut sum = 0;

    for n in (1..vec.len()){
        println!("{}-{}:{}",vec[n-1],vec[n],vec[n-1]<vec[n]);
        if vec[n-1]<vec[n] {
            sum += 1;
        }
    }

    println!("sum is {}", sum);
    
}