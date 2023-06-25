#[derive(Debug)]
pub struct ShoppingItem {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct ShoppingList {
    pub title: String,
    pub items: Vec<ShoppingItem>,
}
