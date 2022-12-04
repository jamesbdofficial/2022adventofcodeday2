pub mod file_to_vector;
pub mod who_won;

fn main() {
    let rps_cheat_sheet_vector: Vec<String> = file_to_vector::text_to_vec();
    let mut iterator: usize = 0;
    let mut enemy_move: String = "".to_string();
    let mut game_outcome: String = "".to_string();
    let mut total_points: u64 = 0;

    for _number in rps_cheat_sheet_vector.iter() {
        let split = rps_cheat_sheet_vector[iterator].split(" ");
        let mut spliterator: usize = 0;
        let mut points: u64 = 0;
        for s in split {
             if spliterator == 0 {
                 enemy_move = s.to_string();
             } else if spliterator == 1 {
                game_outcome = s.to_string();
             }
             spliterator+=1;
        }

        if *game_outcome == "X".to_string() {
            // Do nothing, as you need to lose.
        } else if *game_outcome == "Y".to_string() {
            points+= 3;
        } else if *game_outcome == "Z".to_string() {
            points+= 6;
        }


        let your_move: u64 = who_won::win_calculator(&enemy_move, &game_outcome);

        if your_move == 1 {
            points+= 1; // You used rock.
        } else if your_move == 2 {
            points+= 2; // You used paper.
        } else if your_move == 3 {
            points+= 3; // You used scissors.
        }

        total_points+= points;
        iterator+=1;
    }

    println!("Your total points were: {}", total_points);
}
