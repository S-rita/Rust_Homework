fn main(){
    let args: Vec<String> = std::env::args().collect();

        if args.len() != 4 {
            eprintln!("Please enter three arguments! (min Fahr, max Fahr, range)");
            std::process::exit(1);
        }

    let x: f32 = args[1].parse().unwrap();
    let y: f32 = args[2].parse().unwrap();
    let z: f32 = args[3].parse().unwrap();

    let result1:f32 = Rust_hw3_2::calc1(x);
    let result2:f32 = Rust_hw3_2::calc2(x, z);
    let result3:f32 = Rust_hw3_2::calc3(x, z);
    let result4:f32 = Rust_hw3_2::calc4(y, z);
    let result5:f32 = Rust_hw3_2::calc5(y, z);
    let result6:f32 = Rust_hw3_2::calc6(y);

    let result1_1:f32 = Rust_hw3_2::calc1_(x);
    let result2_1:f32 = Rust_hw3_2::calc2_(x, z);
    let result3_1:f32 = Rust_hw3_2::calc3_(x, z);
    let result4_1:f32 = Rust_hw3_2::calc4_(y, z);
    let result5_1:f32 = Rust_hw3_2::calc5_(y, z);
    let result6_1:f32 = Rust_hw3_2::calc6_(y);

    if x < y {
        println!("Fahr\tCelcius");
        println!("{}\t{:.1}", x, result1);
        println!("{}\t{:.1}", x+z, result2);
        println!("{}\t{:.1}", x+(z*2.0), result3);
        println!(".....");
        println!("{}\t{:.1}", y-(z*2.0), result4);
        println!("{}\t{:.1}", y-z, result5);
        println!("{}\t{:.1}", y, result6);
    }
    else if x > y {
        println!("Fahr\tCelcius");
        println!("{}\t{:.1}", x, result1_1);
        println!("{}\t{:.1}", x-z, result2_1);
        println!("{}\t{:.1}", x-(z*2.0), result3_1);
        println!(".....");
        println!("{}\t{:.1}", y+(z*2.0), result4_1);
        println!("{}\t{:.1}", y+z, result5_1);
        println!("{}\t{:.1}", y, result6_1);
    }
}
