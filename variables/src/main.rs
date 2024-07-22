fn main() {
    let mut x = 3;
    println!("The value of x is {x}");
    x = 5;
    println!("The value of x is {x}");

    const SPEED_OF_LIGHT: u32 = 299792458;

    println!("Speed of light {SPEED_OF_LIGHT}");


    let xx = 5;
    
    let xx = xx + 2;

    {
        let xx = xx *3;
        println!("Shadow xx is {xx}");
    }

    println!("XX is {xx}");

    let spaces = "   ";
    let spaces = spaces.len();
    //shadow 可以改变类型

    println!("The size of spaces is {spaces}");

}
