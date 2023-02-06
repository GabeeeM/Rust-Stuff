use std::io;

fn main() {
    println!("what number do you want to go to");

    loop {
        let mut place = String::new();

        io::stdin().read_line(&mut place).expect("Failed to read line");

        let place: u32 = match place.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if place < 2 {
            println!("{}", 1);
            break;
        }

        let mut fibo: Vec<u32> = vec![1,1];

        for x in 1..=place {
            fibo.push(fibo[x as usize] + fibo[(x as usize) - 1]);
        }

        println!("{}", fibo[place as usize]);
        break;

    }   
}
