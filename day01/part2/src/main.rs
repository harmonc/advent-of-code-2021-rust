use std::fs;
use std::str;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("in.txt")
        .expect("Something went wrong reading the file");
    

    let vec: Vec<i32> = contents.split("\r\n").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut sum = 0;

    for n in (3..vec.len()){
        let a = vec[n-1] + vec[n-2] + vec[n-3];
        let b = vec[n] + vec[n-1] + vec[n-2];
        println!("{}-{}:{}",a,b,a<b);
        if a < b {
            sum += 1;
        }
    }

    println!("sum is {}", sum);
    
}