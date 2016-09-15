enum NumberOrNothing {
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number, Nothing};
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {

    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b {a} else {b}
    }

    let mut min = Nothing;
    for el in vec {
        min = Number(match min {
            Nothing =>  el,
            Number(n) => min_i32(n, el)
        })
    }

    min
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 3, 9, 27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
