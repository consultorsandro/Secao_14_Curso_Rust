mod inventory;
mod orders;

fn main() {
   println!("Our managers are {} and {}. We have {} square feet of floor space.", 
            inventory::MANAGER,
            orders::MANAGER,
            inventory::FLOOR_SPACE
   );
    inventory::talk_to_manager();

    let favorite_category = inventory::products::ProductCategory::Hammer;
    println!("My favorite category is: {:?}", favorite_category);

    let tall_ladder = inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 10,
    };  
    println!("{:#?}", tall_ladder);
}