use core::num;
use std::io;

const SPEED_OF_LIGHT: u64 = 299_792_458;

fn time_to_travel(distance: u64) -> f64 {
    distance as f64 / SPEED_OF_LIGHT as f64
}
fn main() {
    // const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;
    // const PI: f64 = 3.14159;
    // const CIRCUMFERENCE: f64 = 2.0 * PI * 5.0;

    // // println!("{FIVE_HOURS_IN_SECONDS}");
    // // println!("{PI}");
    // // println!("{CIRCUMFERENCE}");

    // let distance = 1_000_000_000;
    // let time = time_to_travel(distance);
    // // println!("Time to travel {distance} meters at the speed of light: {time:.6} seconds");

    // let spaces = "hello";
    // let spaces = 8;

    // // println!("{spaces}");

    // let quotient = 56.7 / 32.2;
    // let truncated: i32 = -5 / 3;

    // let rm = 40 % 5;

    // // println!("{rm}")

    // let tup = (500, 56.6, 1);
    // let (x, y, z) = tup;

    // // println!("{y}");

    // let five_hundred = tup.0;
    // let one = tup.2;

    // // println!("{one} and {five_hundred}");

    // let a: [i32; 5] = [1, -5, 3, 4, 5];

    // // let a = [10; 5];

    // // println!("{}", a[1]);

    // loop {
    //     let mut index = String::new();

    //     println!("input the index value:");
    //     io::stdin().read_line(&mut index).expect("failed to read");
    //     let index: usize = match index.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please input a valid number.");
    //             continue;
    //         }
    //     };

    //     if index >= a.len() {
    //         println!("please input a smaller value");
    //         continue;
    //     } else {
    //         let element = a[index];
    //         println!("The value of the element at index {index} is: {element}");
    //         break;
    //     }
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    // for number in (1..=4).rev() {
    //     println!("{number}");
    // }

    struct Person {
        name: String,
        age: i32,
        is_alive: bool,
    }

    let p1 = Person {
        name: String::from("Mae Mann"),
        age: 43,
        is_alive: false,
    };

    fn personBuilder(name: String, age: i32, is_alive: bool) -> Person {
        Person {
            name,
            age,
            is_alive,
        }
    }

    let p2 = personBuilder(String::from("Alejandro Hines"), 37, true);

    println!("Name: {} Age: {} Is alive: {}", p1.name, p1.age, p1.is_alive);

    impl Person {
        fn info(&self){
            println!("Name: {} Age: {} Is alive: {}", self.name, self.age, self.is_alive)
        }
    }

    p2.info();
        
}
