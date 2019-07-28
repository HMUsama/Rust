pub fn run(){
    // Print To console
    println!("Hello from the println.rs File");
    
    // Basic Formatting
    println!("{} is from {}","Usama","Umer");

    // Positional Arguments
    println!("{} is from {} and {} liks to {2}","Usama","Umer","Code" );

    // Name Arguments
    println!("{name} liks to play {activity}",
    name="Usama",
    activity="Umer" );

    // Placeholder traids
    println!("Binary : {:b} Hex: {:x} Octal: {:o}",10,10,10, )
}