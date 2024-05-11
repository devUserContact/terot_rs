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
const FACES: [&str; 4] = ["Page", "Knight", "Queen", "King"];
const SUITS: [&str; 4] = ["Cups", "Wands", "Penticles", "Swords"];

#[derive(Debug)]
pub struct MinorArcana {
    cups: Vec<u8>,
    wands: Vec<u8>,
    penticles: Vec<u8>,
    swords: Vec<u8>,
}
impl MinorArcana {
    pub fn add_suits_numbers(&mut self, number: &u8) {}
}

pub fn load_cards() {
    let minor_arcana_values: Vec<u8> = (1..11).collect();
    let mut terrot_deck = MinorArcana {
        cups: Vec::new(),
        wands: Vec::new(),
        penticles: Vec::new(),
        swords: Vec::new(),
    };
    for number in minor_arcana_values.iter() {
        terrot_deck.cups.push(number.clone());
        terrot_deck.wands.push(number.clone());
        terrot_deck.penticles.push(number.clone());
        terrot_deck.swords.push(number.clone());
    }
    println!("{:?}", terrot_deck)
}
pub fn shuffle() {}
pub fn draw_cards() {}
