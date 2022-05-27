pub struct WoodFurniture {
    pub skin: String,
    pub price: u32
}

pub struct IronFurniture {
    pub skin: String,
    pub price: u32
}

pub trait Furniture {
    fn new() -> Self;
    fn getSkin(&self) -> String;
    fn getPrice(&self) -> u32;
}

impl Furniture for WoodFurniture {
    fn new() -> Self {
        WoodFurniture { skin: String::from("Wood"), price: 999 }
    }

    fn getSkin(&self) -> String {
        println!("This is wood furniture!");
        let result = String::from("Wood Skin");
        return result;
    }

    fn getPrice(&self) -> u32 {
        self.price
    }
}

impl Furniture for IronFurniture {

    fn new() -> Self {
        IronFurniture { skin: String::from("Iron"), price: 1111 }
    }

    fn getSkin(&self) -> String {
        println!("This is iron furniture!");
        let result = String::from("Iron Skin");
        return result;
    }

    fn getPrice(&self) -> u32 {
        self.price
    }
}

fn main() {
    println!("\n");
    let iron: IronFurniture = Furniture::new();
    println!("skin = {} \n price = {}", iron.getSkin().to_string(), iron.getPrice());
    println!("\n===\n");
    let wood: WoodFurniture = Furniture::new();
    println!("skin = {} \n price = {}", wood.getSkin().to_string(), wood.getPrice());
}