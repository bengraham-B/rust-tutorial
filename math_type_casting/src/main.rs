use std::io;
fn main() {
    let x: f64 = 255.0; // 0 <-> 255
    let y: f64 = 10.0; // -182 <-> 127



    // let z = x + y;
    let p = x / y;
    println!("z : {}", p);

    // mod
    let p = x % y;
    println!("Mod operator, remainder: of {} & {}: {}", x, y, p);

    // Another way to right this
    let g = 127_i64;
    let f = 10 as i64;
    let c = f + g;

    println!("{} + {} = {}", f, g, c);

    // Chaning types
    let x = 127_000 as i64; //& This makes it more resonable
    let y = 10_i32;
    let k = 169_000_000 as i64;

    let z = x / (y as i64);
    let zzTop = z + k;

    println!("z: {}", z);
    println!("zzTop: {}", zzTop);

    //* Conver user Input to a number */
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ex[ected to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);

}
