fn fib(n: u32) -> u32 {
    if n < 2 { return n }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let n = 11;
    let result = fib(n);
    println!("Fibonacci #{} is {}", n, result);
}
