fn main() {

    // VARIABLES
    // by default variables are immutable
    let mut x = 33;
    println!("The Number X is: {}", x);
    x = 5;
    println!("The Number X is {}", x);

    // never mutate a const variable
    const COUNT: u32 = 500_000;
    //COUNT = 4;
    println!("The Counter is {}", COUNT);
}
