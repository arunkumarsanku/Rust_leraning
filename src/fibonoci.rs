// write a function fib that finds the fibonacci number it takes as input
// default is const

fn main() {
    println!("{}", fib(10));
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 2..num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
