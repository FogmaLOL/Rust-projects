mod AntiVM; // starts the mod for AntiVM

fn main() {
    if !AntiVM::antivm() { // if it finds thats it not a VM it prints hello world
        println!("Hello, World!");
    } else { // if it finds it is a VM it prints Ur in a VM 
        println!("Ur in a VM");
    }
}