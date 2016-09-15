enum NumberOrNothing {
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number, Nothing};

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;
    for el in vec {
        min = match min {
            Nothing =>  Number(el),
            Number(n) => Number(min_i32(n, el))
        }
    }

    min
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {a} else {b}
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 3, 9, 27]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("The number is: <nothing>"),
        Number(n) => println!("The number is: {}", n),
    };
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}
