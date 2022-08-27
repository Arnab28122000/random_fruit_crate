//! # My Crate
//! 
//! `get_rand_fruits` helps you generate random fruits and takes as an argument
//! Generating fruits are more convinient.

/// Generates random fruits
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = rand_fruits::generate_fruits(arg);
/// assert_eq!(5, answer.len());
/// ```
use rand::prelude::*;

pub fn generate_fruits(n: i32) -> Vec<&'static str> {
    let mut rng = thread_rng();
    let fruits= vec![ "Apple",
    "Apricot",
    "Avocado",
    "Banana",
    "Boysenberrie",
    "Blueberrie",
    "Bing Cherry",
    "Cherrie",
    "Cantaloupe",
    "Crab apple",
    "Clementine",
    "Cucumber",
    "Damson plum",
    "Dinosaur Eggs",
    "Date",
    "Dewberrie",
    "Dragon Fruit",
    "Elderberry",
    "Eggfruit",
    "Evergreen Huckleberry",
    "Entawak",
    "Fig",
    "Farkleberry",
    "Finger Lime",
    "Grapefruit",
    "Grape",
    "Gooseberrie",
    "Guava",
    "Honeydew melon",
    "Hackberry",
    "Honeycrisp Apple",
    "Indian Prune",
    "Indonesian Lime",
    "Imbe",
    "Indian Fig",
    "Jackfruit",
    "Java Apple",
    "Jambolan",
    "Kiwi",
    "Kaffir Lime",
    "Kumquat",
    "Lime",
    "Longan",
    "Lychee",
    "Loquat",
    "Mango",
    "Mandarin Orange",
    "Mulberry",
    "Melon",
    "Nectarine",
    "Navel Orange",
    "Nashi Pear",
    "Olive",
    "Orange",
    "Ogeechee Lime",
    "Oval Kumquat",
    "Papaya",
    "Persimmon",
    "Paw Paw",
    "Prickly Pear",
    "Peach",
    "Pomegranate",
    "Pineapple",
    "Quince",
    "Queen Anne Cherry",
    "Quararibea cordata",
    "Rambutan",
    "Raspberrie",
    "Rose Hip",
    "Star Fruit",
    "Strawberrie",
    "Sugar Baby Watermelon",
    "Tomato",
    "Tangerine",
    "Tamarind",
    "Tart Cherrie",
    "Ugli Fruit",
    "Uniq Fruit",
    "Ugni",
    "Vanilla Bean",
    "Velvet Pink Banana",
    "Voavanga",
    "Watermelon",
    "Wolfberry",
    "White Mulberry",
    "Xigua",
    "Ximenia caffra fruit",
    "Xango Mangosteen Fruit Juice",
    "Yellow Passion Fruit",
    "Yunnan Hackberry",
    "Yangmei",
    "Zig Zag Vine fruit",
    "Zinfandel Grape",
    "Zucchini"];
    let mut num = 94;
    if num >= n {
        num = n;
    }
    let mut generated_vec : Vec<&str> = vec![];
    for _i in 0..num{
        let t = rng.gen_range(0..94);
        generated_vec.push(match fruits.get(t){
            Some(str) =>  str,
            None => "Not Found"
        });
    }

    generated_vec
}
