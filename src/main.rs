use rand::Rng;
use std::io;

fn pull_card(mut score : i32, mut deck : Vec<i32> ) {

    println!("You're taking the card...");
    let random_pick = rand::random_range(0..(deck.len())-1);
    score += deck[random_pick];
    deck.remove(random_pick);
    println!("You took card with score {card}, you score is {score}", card=deck[random_pick]);

}
fn game() {

    println!("Game starts...");

    let mut deck = vec![1, 1, 1, 1,
                        2, 2, 2, 2,
                        3, 3, 3, 3,
                        4, 4, 4, 4,
                        5, 5, 5 ,5,
                        6, 6, 6, 6,
                        7, 7, 7, 7,
                        8, 8, 8, 8,
                        9, 9, 9, 9,
                        10, 10, 10, 10,
                        10, 10, 10, 10,
                        10, 10, 10, 10,
                        10, 10, 10, 10];

    let mut dealer_score = 0;
    let mut player_score = 0;
    let mut dealer_last_pick = 0;
    let mut turns = 0;

    println!("Dealer is taking 2 cards...");

    let mut random_pick = rand::random_range(0..(deck.len())-1);

    dealer_score += deck[random_pick];
    deck.remove(random_pick);

    random_pick = rand::random_range(0..(deck.len())-1);

    dealer_last_pick = deck[random_pick];
    dealer_score += deck[random_pick];
    deck.remove(random_pick);

    println!("Dealer's score is {dealer_score}, his last card is {dealer_last_pick}");

    println!("You are taking 2 cards...");
    random_pick = rand::random_range(0..(deck.len())-1);
    println!("You taked a card with score: {card}", card = deck[random_pick]);
    player_score += deck[random_pick];
    deck.remove(random_pick);

    random_pick = rand::random_range(0..(deck.len())-1);
    println!("You taked a card with score: {card}", card = deck[random_pick]);
    player_score += deck[random_pick];
    deck.remove(random_pick);


    loop {
        let deck_cards = deck.len();

        if player_score > 21{

            println!("You score is more than 21 cards. You lost, dealer wins");
            break;

        } else if dealer_score > 21 {

            println!("Dealer's score is more than 21 cards. You won!");
            break;

        } else if player_score == 21 {

            println!("It's blackjack. You won!");
            break;

        } else if dealer_score == 21 {

            println!("It's dealer's blackjack. You lost.");
            break;

        } else if dealer_score >= 17 && turns != 0 {

            if dealer_score > player_score {

                println!("You lost, dealer wins.");
                break;

            } else if dealer_score < player_score {

                println!("You won!");
                break;

            } else {

                println!("Nobody wins.");
                break;
            }

        }

        loop {

            println!("Turn {turn}", turn = turns + 1);
            println!("Count of cards in deck: {deck_cards}");
            println!("Your score {player_score}, dealer score {dealer_score}, his last card is {dealer_last_pick}");
            println!("Select your action: H - Hit, S - Stand");

            let mut player_pick = String::new();
            io::stdin()
                .read_line(&mut player_pick)
                .expect("Failed to read line");

            if player_pick.trim() == "H" {

                println!("You are taking the card...");
                random_pick = rand::random_range(0..deck_cards-1);
                println!("You took a card with score: {card}", card = deck[random_pick]);
                player_score += deck[random_pick];
                deck.remove(random_pick);

                turns += 1;
                break;

            } else if player_pick.trim() == "S" {

                println!("You standing, it's dealer turn.");

                loop {
                    if dealer_score >= 17 {

                        println!("Dealer's turn is over.");
                        break;

                    } else {

                        random_pick = rand::random_range(0..(deck.len())-1);
                        dealer_score += deck[random_pick];
                        deck.remove(random_pick);
                        println!("Dealer took the card, it`s {card}.", card = deck[random_pick]);
                        println!("Dealer's score: {dealer_score}.");

                    }

                    turns += 1;

                }

                if dealer_score >= 17 {

                    break;
                }


            } else {

                println!("Action is not defined");

            }


        }

    }

}

fn main() {

        game();
    loop { } // will be fixed
}
