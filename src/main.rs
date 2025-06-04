mod inventory { //class 224 - 228
    const FLOOR_SPACE: i32 = 1000;
    pub const MANAGER: &str = "Ivan Inventory";

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
}

mod orders {
    pub const MANAGER: &str = "Ivan Orders";
}
fn main() {
    println!("The manager is: {}", inventory::MANAGER);
    println!("The manager is: {}", orders::MANAGER);
}
