trait Example {
    fn zero(&self) -> i32 {
        0
    }
}

struct One;
struct Two;

impl Example for One {
    fn zero(&self) -> i32 {
        1
    }
}

impl Example for Two {
    fn zero(&self) -> i32 {
        2
    }
}

fn vector_of_traits() {
    let a: Vec<Box<dyn Example>> = vec![Box::new(One), Box::new(Two)];
    let _b: Vec<i32> = a.into_iter().map(|x| x.zero()).collect();
}
