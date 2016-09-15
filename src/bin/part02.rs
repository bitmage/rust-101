enum SomethingOrNothing<T> {
    Something(T),
    Nothing
}
use self::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b {self} else {b}
    }
}


fn vec_min<T: Minimum>(vec: Vec<T>) -> SomethingOrNothing<T> {

    let mut min = Nothing;
    for e in vec {
        min = Something(match min {
            Nothing =>  e,
            Something(n) => e.min(n)
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
