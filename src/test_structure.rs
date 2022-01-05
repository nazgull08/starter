#[allow(dead_code)]

fn main() {

    let mut char1 = HeroChar {
        name: "Bob".to_string(),
        hp: 50,
        armor: 100, 
        dmg: 10,
        speed: 2.66,
        inventory_size: 5
    };

    println!("name = {}", char1.name);
    char1.hp = 10;
    println!("hp = {}", char1.hp);
    println!("armor = {}", char1.armor);
    println!("dmg = {}", char1.dmg);
    println!("speed = {}", char1.speed);
    println!("inventory_size = {}", char1.inventory_size);
}
struct HeroChar {
    name: String,
    hp: u8,
    armor: u8,
    dmg: u8,
    speed: f32,
    inventory_size: u8,
}
