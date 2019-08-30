pub struct Commodity {
    pub id: u32,
    pub name: String,
}

pub struct SelectedCommodity {
    pub item: Commodity,
    pub num: u32,
}

pub struct Order {
    pub price: u32,
    pub items: Vec<SelectedCommodity>,
}