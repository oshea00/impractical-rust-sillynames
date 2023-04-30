use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::io::{stdin, stdout};

const FIRST_NAMES: [&str; 68] = [
    "Baby Oil",
    "Bad News",
    "Big Burps",
    "Bill 'Beenie-Weenie'",
    "Bob 'Stinkbug'",
    "Bowel Noises",
    "Boxelder",
    "Bud 'Lite'",
    "Butterbean",
    "Buttermilk",
    "Buttocks",
    "Chad",
    "Chesterfield",
    "Chewy",
    "Chigger",
    "Cinnabuns",
    "Cleet",
    "Cornbread",
    "Crab Meat",
    "Crapps",
    "Dark Skies",
    "Dennis Clawhammer",
    "Dicman",
    "Elphonso",
    "Fancypants",
    "Figgs",
    "Foncy",
    "Gootsy",
    "Greasy Jim",
    "Huckleberry",
    "Huggy",
    "Ignatious",
    "Jimbo",
    "Joe 'Pottin Soil'",
    "Johnny",
    "Lemongrass",
    "Lil Debil",
    "Longbranch",
    "'Lunch Money'",
    "Mergatroid",
    "'Mr Peabody'",
    "Oil-Can",
    "Oinks",
    "Old Scratch",
    "Ovaltine",
    "Pennywhistle",
    "Pitchfork Ben",
    "Potato Bug",
    "Pushmeet",
    "Rock Candy",
    "Schlomo",
    "Scratchensniff",
    "Scut",
    "Skidmark",
    "Slaps",
    "Snakes",
    "Snoobs",
    "Snorki",
    "Soupcan Sam",
    "Spitzitout",
    "Squids",
    "Stinky",
    "Storyboard",
    "Sweet Tea",
    "TeeTee",
    "Wheezy Joe",
    "Winston 'Jazz Hands'",
    "Worms",
];

const LAST_NAMES: [&str; 65] = [
    "Appleyard",
    "Bigmeat",
    "Bloominshine",
    "Boogerbottom",
    "Breedslovetrout",
    "Butterbaugh",
    "Clovenhoof",
    "Clutterbuck",
    "Cocktoasten",
    "Endicott",
    "Fewhairs",
    "Gooberdapple",
    "Goodensmith",
    "Goodpasture",
    "Guster",
    "Henderson",
    "Hooperbag",
    "Hoosenater",
    "Hootkins",
    "Jefferson",
    "Jenkins",
    "Jingley-Schmidt",
    "Johnson",
    "Kingfish",
    "Listenbee",
    "M'Bembo",
    "McFadden",
    "Moonshine",
    "Nettles",
    "Noseworthy",
    "Olivetti",
    "Outerbridge",
    "Overpeck",
    "Overturf",
    "Oxhandler",
    "Pealike",
    "Pennywhistle",
    "Peterson",
    "Pieplow",
    "Pinkerton",
    "Porkins",
    "Putney",
    "Quakenbush",
    "Rainwater",
    "Rosenthal",
    "Rubbins",
    "Sackrider",
    "Snuggleshine",
    "Splern",
    "Stevens",
    "Stroganoff",
    "Sugar-Gold",
    "Swackhamer",
    "Tippins",
    "Turnipseed",
    "Vinaigrette",
    "Walkingstick",
    "Wallbanger",
    "Weewax",
    "Weiners",
    "Whipkey",
    "Wigglesworth",
    "Wimplesnatch",
    "Winterkorn",
    "Woolysocks",
];

fn main() {
    let mut rng = SmallRng::from_entropy();
    let how_many = input("How many names? ")
        .parse::<u32>()
        .unwrap_or_else(|_| 1);
    println!("{how_many} funny names:");
    for _ in 0..how_many {
        let first = FIRST_NAMES.iter().choose(&mut rng).unwrap();
        let last = LAST_NAMES.iter().choose(&mut rng).unwrap();
        write_styled(format!("{first} {last}\n"), Color::Reset, Color::Magenta);
    }
}

fn write_styled(line: String, background: Color, foreground: Color) {
    execute!(
        stdout(),
        SetForegroundColor(foreground),
        SetBackgroundColor(background),
        Print(line),
        ResetColor
    )
    .unwrap();
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    write_styled(String::from(prompt), Color::Reset, Color::Reset);
    stdin().read_line(&mut input).expect("success");
    String::from(input.trim())
}
