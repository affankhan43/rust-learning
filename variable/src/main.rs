fn main() {

    // VARIABLES
    // by default variables are immutable
    let mut x = 33;
    println!("The Number X is: {}", x);
    x = 5;
    println!("The Number X is {}", x);

    // SHADOWING
    let y = 33;
    println!("The Number Y is: {}", y);
    let y = "SIX";
    println!("The Number Y is {}", y);

    // never mutate a const variable
    const COUNT: u32 = 500_000;
    //COUNT = 4;
    println!("The Counter is {}", COUNT);


    //INTEGERS
    let a = 98_222; //Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; //Byte (u8 Only)

    let f: u8 = 255; //max u8 is 255
    println!( "The Number f is {}" , f);

    //FLOATING-POINT NUMBERS
    let g: f64 = 44.23;
    let h: f32 = 3.0;

    //BOOLEAN
    let i: bool = true;
    let j: bool = false;

    //CHAR
    let k = 'T';
    let l = '😀';
    println!("The Charachter l is {}", l);

}
