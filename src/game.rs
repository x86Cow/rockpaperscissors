use std::io;
use player::Player;

#[path ="./player.rs"]
pub(crate) mod player;




pub struct GameInfo {
   pub player_one_turn: bool,
   pub turn_num: i32,
}

/**
 * Perams: player: player struct; turn_num: whose turn is it.
 */
pub fn player_input(player:&mut Player, turn_num: i32 ) -> io::Result<()> {

    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    println!("Player {}, please pick an option: Rock, Paper, Scissors", turn_num);
    stdin.read_line(&mut input)?;

    player.player_selection = input.to_ascii_lowercase().replace('\n', "");

    Ok(())
}
pub fn wins(player1: &Player, player2: &Player) {
    if player1.player_selection == "rock" {
        if player2.player_selection == "paper" {
            println!("Player Two Wins!");
        } else if player2.player_selection ==  "scissors" {
            println!("Player One Wins!");
        } else if player2.player_selection == "rock" {
            println!("Draw!");
        }
    } else if player1.player_selection == "paper" {
        if player2.player_selection == "rock" {
            println!("Player One Wins!");
        } else if player2.player_selection == "scissors" {
            println!("Player Two Wins!");
        } else if player2.player_selection == "paper" {
            println!("Draw!");
        }
    } else if player1.player_selection == "scissors" {
        if player2.player_selection == "rock" {
            println!("Player One Wins!");
        } else if player2.player_selection == "paper" {
            println!("Player One Wins!");
        } else if player2.player_selection == "scissors" {
            println!("Draw!");
        }
    }

}