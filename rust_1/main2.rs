//primitive dtypes
//signed int = i8,i16,i32,i64
//unsigned int = u8,u16.......
fn main(){
    let x: i32 = -67;
    let y: u64 = 69;
    println!("Sined Int: {}",x);
    println!("Unsigned Int: {}",y);

    // range of i32 = -2^31 =>2^31 (min max)
    // range of u64 = 2^63(min max)
    let e: i32 = 2147483648;
    println!("Error{}",e)
//  note: the literal `2147483648` does not fit into the type `i32` whose range is `-2147483648..=2147483647`
//    = help: consider using the type `u32` instead
//    = note: `#[deny(overflowing_literals)]` on by default
}