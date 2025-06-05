pub mod products;
// File: src/inventory.rs

pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Alice";

pub fn talk_to_manager() {
    println!("{} Hey, how's your coffee? What do you think of {:?}", 
    MANAGER, 
    ProductCategory::Ladder);

}
