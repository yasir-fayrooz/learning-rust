

fn main(){
    //By default, any {} will be automatically replaced with any arguments stringified.
    println!("{} days", 31);

    //Positional arguments can also be used within {}
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //Args can be named too
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    //Different formating options can be given
    println!("Base 10:      {}", 69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hex):        {:x}", 69420);
    println!("Base 16 (hex):        {:X}", 69420);
    
    //You can right-justify text with a specific width of whitespace
    println!("{number:>5}", number=1);
    //pad with extra 0s
    println!("{number:0>5}", number=1);
    //and left-adjust by flipping the sign
    println!("{number:0<5}", number=1);
    
    //you can use named args in the format specifier by appending $
    println!("{number:0>width$}", number=1, width=5);

    //Rust even checks to make sure the correct number of args are used!
    //println!("{0}, {1}", "Bond");

    //Only types that implement fmt::Display can be formatted with '{}'. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)]
    struct Structure(i32);

    //This will not compile because `Structure` does not implement
    // fmt::Display
    //println!("This struct `{}` wont print....", Structure(3));
    //TODO ^ Try uncommenting this line and fix

    //For Rust 1.58 and above, you can directly capture the argument from a 
    //surrounding variable. Just like the above, this will output
    // "    1", 4 whitspaces and a 1.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}