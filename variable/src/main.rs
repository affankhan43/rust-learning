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

    //Compound Types (TUPLE)
    let tup = ("Let's Get Rusty", 123, 44.2);
    let tup1  = tup;
    let sub_count = tup1.1;

    let error_codes: [i32; 5] = [200, 202, 404, 500, 503];
    let not_found = error_codes[2];

    let set = [2; 8];

    //addition
    let sum1 = 44+66;
    let sum2 = 44.333 +3.1;

    //substraction
    let sub1 = 33-11;
    let sub2 = 31.555 - 123.8;

    //multiplication
    let mul1 = 6 * 5;
    let mul2 = 2.55 * 76.77;

    //division
    let div1 = 56.7 / 9.1;
    println!("The Division Result is {}", div1);

    //reminder
    let rem = 65 % 7;

    let summer = my_function(32, 22);
    println!("The Function Return is {}", summer);
}

fn my_function (x: i32, y: i32) -> i32{
    println!("The X is: {}", x);
    println!("The Y is: {}", y);
    x+y
    //return x+y;
}
