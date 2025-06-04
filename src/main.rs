mod inventory;
mod orders;

use inventory::products::{Item, ProductCategory};
use inventory::{talk_to_manager, FLOOR_SPACE, MANAGER};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );
    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category is: {:?}", favorite_category);

    let tall_ladder = Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 10,
    };
    println!("{:#?}", tall_ladder);
}
