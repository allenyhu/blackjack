use text_io::read;

struct Card {
    suit: char,
    rank: String,
    value: u8,
    count_value: u8,
}

struct Deck {
    cards: Vec<Card>,
}

fn main() {
    let suits = ['C', 'H', 'S', 'D'];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    let mut deck: Deck = Deck {
        cards: Vec::<Card>::new(),
    };

    for suit in suits {
        for rank in ranks {
            let card = Card {
                suit,
                rank: String::from(rank),
                value: 0,
                count_value: 0,
            };
            deck.cards.push(card)
        }
    }

    println!("Deck size: {}", deck.cards.len());

    loop {
        println!("How much do you want to bet?");

        let bet: i32 = read!();
        println!("You bet ${}", bet)
    }
}
