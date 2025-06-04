mod inventory;
mod orders;

fn main() {
    println!("The manager is: {}", inventory::MANAGER);
    println!("The manager is: {}", orders::MANAGER);
}