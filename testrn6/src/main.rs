fn main() {
    /*println!("{number:0>5}", number=1);
    println!("{number:>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
    println!("My name is {0}, {1} {0}", "Bond", "James");*/
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    println!("{x:0>5}", x=1);
    println!("{x:0>y$}", x=1, y=5);
    let x: f64=1.0;
    let w: usize=5;
    println!("{x:>w$}");
}
