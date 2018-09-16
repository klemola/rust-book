type Kilometers = i32;
type Thunk = Box<Fn() + Send + 'static>;

fn main() {
    let km: Kilometers = 9001;
    let thunk: Thunk = new_thunk();
    run_thunk(thunk);
    println!("Lots of kilometers: {}", km);

    // Dynamically sized types
    let string_example: String = String::from("String");
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
    let s3: &str = &string_example;

    println!("Slices are \"{}\", \"{}\" and \"{}\"", s1, s2, s3);

    generic("Hi");
    relaxed_generic(&42);
}

fn run_thunk(t: Thunk) {
    t();
}

fn new_thunk() -> Thunk {
    return Box::new(|| println!("hi"));
}

fn generic<T: Sized>(t: T)
where
    T: std::fmt::Display,
{
    println!(
        "I am a generic function and my parameter is of unknown size: {}",
        t
    );
}

fn relaxed_generic<T: ?Sized>(t: &T)
where
    T: std::fmt::Display,
{
    println!(
        "I am a relaxed generic function and my parameter is of fixed size: {}",
        t
    );
}
