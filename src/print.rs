// pub is public and fn is function
pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Number: {}",1);
    println!("My name is {} {}.", "Rishabh", "Singh");
    println!("{} is a {}.", "This", "Cat");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code"); // indexing Brad->0, Mass->1, code->2

    // named arguments
    println!("{name} likes to play {activity}", name = "Rishabh", activity = "Football"); 

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10, 10, 10);

    // placeholder for debug trait
    println!("{:?}",(12, true, "hello"));

    // basic mathematics
    println!("10 + 10 = {}",10 + 10);
    
}