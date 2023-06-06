use game::*;

#[path ="./game.rs"]
mod game;

#[path = "./player.rs"]
mod player;

fn main(){
    let mut gameInfo = game::GameInfo {
        player_one_turn: true,
        turn_num: 1,
    };

    let mut player1 = player::Player {
        player_selection: "".to_string(),
    };

    let mut player2 = player::Player {
        player_selection: "".to_string(),
    };

    loop {
        if gameInfo.player_one_turn {
            gameInfo.turn_num = 1;
            player_input(&mut player1, gameInfo.turn_num);
            gameInfo.turn_num = 2;
        } else {
            gameInfo.turn_num = 2;
            player_input(&mut player2, gameInfo.turn_num);
            gameInfo.turn_num = 1;
        }
        gameInfo.player_one_turn = !gameInfo.player_one_turn;
        
        game::wins(&player1, &player2);
    }

}
