use game::*;

#[path ="./game.rs"]
mod game;

// #[path = "./player.rs"]
// mod player;

fn main(){
    let mut game_info = game::GameInfo {
        player_one_turn: true,
        turn_num: 1,
    };

    let mut player1 = game::player::Player {
        player_selection: "".to_string(),
    };

    let mut player2 = player::Player {
        player_selection: "".to_string(),
    };

    loop {
        if game_info.player_one_turn {
            game_info.turn_num = 1;
            player_input(&mut player1, game_info.turn_num);
            game_info.turn_num = 2;
        } else {
            game_info.turn_num = 2;
            player_input(&mut player2, game_info.turn_num);
            game_info.turn_num = 1;
        }
        game_info.player_one_turn = !game_info.player_one_turn;
        
        game::wins(&player1, &player2);
    }

}
