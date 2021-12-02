use std::fs;

fn main() {
    let contents = fs::read_to_string("in.txt")
    .expect("Something went wrong reading the file");

    let dirs: Vec<&str> = contents.split("\r\n").collect();

    let mut x = 0;
    let mut y = 0;
    for n in (0..dirs.len()){
        let d: Vec<&str> = dirs[n].split(" ").collect();
        let num: i32 = d[1].parse::<i32>().unwrap();
        if d[0] == "forward" {
            x += num;
        }
        if d[0] == "up" {
            y -= num;
        }
        if d[0] == "down" {
            y += num;
        }
    }

    println!("{}*{}={}",x,y,x*y);
}
