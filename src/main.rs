use rand::Rng;
use std::io;

struct Card {
    suit: String,
    value: String,
    score: u32,
}
fn value_as_score(value: &str) -> u32 {
    if value == "Ace" {
        1
    } else if value == "King" || value == "Queen" || value == "Jack" {
        10
    } else {
        value.parse::<u32>().unwrap()
    }
}

fn get_card(suit: &str, value: &str) -> Card {
    Card {
        suit: suit.to_string(),
        value: value.to_string(),
        score: value_as_score(value),
    }
}
fn get_deck() -> Vec<Card> {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "King", "Queen", "Jack"];
    let mut deck: Vec<Card> = Vec::new();

    for suit in &suits {
        for value in &values {
            deck.push(get_card(suit, value));
        }

        for value in (2..11) {
            deck.push(get_card(suit, value.to_string().as_str()));
        }
    }

    deck
}

fn rule_check(player: u32, dealer: u32) -> bool {
    if player > 21 {
        println!("You lose, dealer wins.");
        return true;
    } else if player == 21 {
        println!("Blackjack!");
        return true;
    } else if dealer > 21 {
        println!("You win, dealer lose.");
    }


    false
}

fn stand_rule_check(player: u32, dealer: u32) -> bool {

    if dealer == 21 {

        println!("Dealer's blackjack!");
        return true;

    } else if player > dealer {

        println!("You win!");
        return true;

    } else if player < dealer {

        println!("You lose, dealer wins.");
        return true;

    } else if player == dealer {

        println!("You lose, nobody wins.");
        return true;

    }

    false
}

fn player_select() -> String {
    let mut select = String::new();
    io::stdin().read_line(&mut select).unwrap().to_string();

    select.to_lowercase().trim().to_string()
}
fn pull_card(deck: &mut Vec<Card>, mut score: u32) -> u32 {
    let random_pick = rand::random_range(0..deck.len());
    score += deck[random_pick].score;
    println!(
        "Card pulled: {suit}, {value}.",
        suit = deck[random_pick].suit,
        value = deck[random_pick].value
    );
    deck.remove(random_pick);

    score
}
fn game() {
    clean_window();

    let mut deck = get_deck();
    let mut player_score: u32 = 0;
    let mut dealer_score: u32 = 0;

    println!("Game starts.");
    println!("Dealer is taking 2 cards...");
    dealer_score = pull_card(&mut deck, dealer_score);
    dealer_score = pull_card(&mut deck, dealer_score);
    println!("Yoy're taking 2 cards...");
    player_score = pull_card(&mut deck, player_score);
    player_score = pull_card(&mut deck, player_score);

    println!(" ");
    println!("Your score: {player_score}, dealer's: {dealer_score}");

    loop {
        if rule_check(player_score, dealer_score) {
            break;
        }

        println!("Select your action: H - hit, S - stand");
        let action = player_select();

        if action == "h" {
            clean_window();
            println!("You're pulling card...");
            player_score = pull_card(&mut deck, player_score);
            println!("Now you score is: {player_score}, dealer's: {dealer_score}");
        } else if action == "s" {
            clean_window();
            println!("You're standing...");
            println!("Dealer is taking cards...");
            while dealer_score < 17 {
                dealer_score = pull_card(&mut deck, dealer_score);
            }
            println!("Dealer's score is: {dealer_score}");

            if stand_rule_check(player_score, dealer_score) {
                break;
            }
        }
    }
}

fn clean_window() {
    println!("\x1B[2J\x1B[1;1H");
}
fn pause() {
    println!("Type anything to continue...");
    let mut pause_in = String::new();
    io::stdin()
        .read_line(&mut pause_in)
        .expect("Failed to read line");
}

fn main() {
    game();
    pause();
}
