const MAJOR_ARCANA: [&str; 22] = [
    "The Fool",
    "The Magician",
    "The Hight Priestess",
    "The Empress",
    "The Emperor",
    "The Hierophant",
    "The Lovers",
    "The Chariot",
    "Strength",
    "The Hermit",
    "Wheel of Fortune",
    "Justice",
    "The Hanged Man",
    "Death",
    "Temperance",
    "The Devil",
    "The Tower",
    "The Start",
    "The Moon",
    "The Sun",
    "Judgement",
    "The World",
];
const SUITS: [&str; 4] = ["Page", "Knight", "Queen", "King"];

#[derive(Debug)]
pub struct MinorArcana {
    cups: Vec<String>,
    wands: Vec<String>,
    penticles: Vec<String>,
    swords: Vec<String>,
}

pub fn load_cards() {
    let minor_arcana_values: Vec<u8> = (1..11).collect();
    println!("{:?}", minor_arcana_values);
}
pub fn shuffle() {}
pub fn draw_cards() {}
