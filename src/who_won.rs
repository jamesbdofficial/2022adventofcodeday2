fn enemy_move_rock(game_outcome: String) -> u64 {
    let mut your_move: u64 = 0;
    if game_outcome == "X".to_string() {
        your_move = 3; // You used scissors.
    } else if game_outcome == "Y".to_string() {
        your_move = 1; // You used rock.
    } else if game_outcome == "Z".to_string() {
        your_move = 2; // You used paper.
    }
    return your_move;
}

fn enemy_move_paper(game_outcome: String) -> u64{
    let mut your_move: u64 = 0;
    if game_outcome == "X".to_string() {
        your_move = 1; // You used rock.
    } else if game_outcome == "Y".to_string() {
        your_move = 2; // You used paper.
    } else if game_outcome == "Z".to_string() {
        your_move = 3; // You used scissors.
    }
    return your_move;
}

fn enemy_move_scissors(game_outcome: String) -> u64 {
    let mut your_move: u64 = 0;
    if game_outcome == "X".to_string() {
        your_move = 2; // You used paper.
    } else if game_outcome == "Y".to_string() {
        your_move = 3; // You used scissors.
    } else if game_outcome == "Z".to_string() {
        your_move = 1; // You used scissors.
    }
    return your_move;   
}

pub fn win_calculator(enemy_move: &String, game_outcome: &String) -> u64 {
    let mut your_move: u64 = 0;
    let local_enemy_move: String = enemy_move.to_string();
    let local_game_outcome: String = game_outcome.to_string();

    if local_enemy_move == "A".to_string() {
        your_move = enemy_move_rock(local_game_outcome);
    } else if local_enemy_move == "B".to_string() {
        your_move = enemy_move_paper(local_game_outcome);
    } else if local_enemy_move == "C".to_string() {
        your_move = enemy_move_scissors(local_game_outcome);
    }

    return your_move;
}