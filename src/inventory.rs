
const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Alice";

#[derive(Debug)]
enum ProductCategory {
    Ladder,
    Hammer,
}
#[derive(Debug)]
struct Item {
    name: String,
    category: ProductCategory,
    quantity: u32,
}

fn talk_to_manager() {
    println!("{} Hey, how's your coffee?", MANAGER);
}
