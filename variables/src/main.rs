fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple backwards: {} {} {}", tup.2, tup.1, tup.0);
    println!("X x max points {}", x * MAX_POINTS);
    let result = another_function(41);
    println!("another func {}", result);
    println!("conditional {}", conditional(result));

    let mut count = 0;

    loop {
        if count == 5 {
            break
        }

        count = count + 1;
        println!("count is {}", count);
    }
}

fn another_function(num: u32) -> u32 {
    {num + 1}
}

fn conditional(num: u32) -> bool {
    if num > 50 {
        true
    } else {
        false
    }

}
