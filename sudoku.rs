use std::collections::HashMap;

type BoardState = [[(u8, [u8; 9]); 9]; 9];
type Unknowns = HashMap<u8, Vec<(u8, u8)>>;
struct GameState {
  unknowns : Unknowns,
  board : BoardState,
}

fn initialize_state(puzzle : &str) -> GameState {
    let solver_state = [[(9u8, [1,2,3,4,5,6,7,8,9]); 9]; 9];
    let mut unknowns = HashMap::new();
    for remaining_count in 2..10 {
        unknowns.insert(remaining_count, Vec::new());
    }
    let mut game_state = GameState{
        unknowns: unknowns,
        board : solver_state
    };
    for (index, character) in puzzle.chars().enumerate() {
        let row: usize = index / 9;
        let column: usize = index % 9;
        if character != '.' {
            //println!("({}, {}) : {}", row+1, column+1, character);
            let value : u8 = character.to_digit(10).unwrap() as u8;
            assert!(update(&mut game_state, row, column, value, true));
        }
    }
    for row in 0..9 {
        for column in 0..9 {
            let count = game_state.board[row][column].0;
            if count > 1 {
                println!("Adding : ({},{}) to bin {}", row + 1, column+1, count);
                let vector = game_state.unknowns.get_mut(&count).unwrap();
                vector.push((row as u8,column as u8));
            }
        }
    }
    return game_state;
}

fn print_state(game_state : &GameState) {
    // for (&remaining, remaining_vec) in &game_state.unknowns {
    //     println!("For: {}", remaining);
    //     for (row, column) in remaining_vec {
    //         print!("({},{})", row, column);
    //     }
    // }
    for row in game_state.board {
        for (count, values) in row {
            if count == 1 {
                print!("{}, ", values[0]);
            } else {
                print!("., ");
                /*print!("(");
                for index in 0..*count {
                    print!("{}, ", values[index as usize])
                }
                print!(")"); */
            }
        }
        println!("");
    }
}

fn update_guesses(guesses : &mut Unknowns, row : usize, column : usize, new_count : u8) {
    //println!("Asking for ({},{}), Old count {}", row+1, column+1, new_count + 1);
    //for old_count in 2..9 {
    let old_count = new_count + 1;
    if old_count >= 2 {
        let guess_vector = guesses.get_mut(&old_count).unwrap();
        if let Some(index) = guess_vector.iter().position(|value| *value == (row as u8, column as u8)) {
            guess_vector.swap_remove(index);
            if new_count > 1 {
                let guess_vector = guesses.get_mut(&(new_count)).unwrap();
                guess_vector.push((row as u8, column as u8));
            }
        }
    }
}

fn update(game : &mut GameState, row : usize, column : usize, value : u8, is_init : bool) -> bool {
    game.board[row][column] = (1, [value, 0, 0, 0, 0, 0, 0, 0, 0]);

    // Update column
    for target_row in 0..9 {
        if target_row != row {
            let (ref mut count, ref mut entries) = &mut game.board[target_row][column];
            //println!("Checking for {} in ({},{})", value, target_row + 1, column + 1);
            for entry_index in 0..*count {
                if entries[entry_index as usize] == value {
                    //println!("Removing {} from ({},{})", value, target_row + 1, column + 1);
                    if entry_index < (*count -1) {
                        entries[entry_index as usize] = entries[(*count - 1) as usize];
                    }
                    *count-=1;
                    if *count == 0 {
                        return false;
                    }
                    if !is_init {
                        update_guesses(&mut game.unknowns, row, column, *count);
                    }
                    break;
                }
            }
        }
    }

    // Update row
    for target_column in 0..9 {
        if target_column != column {
            let (ref mut count, ref mut entries) = &mut game.board[row][target_column];
            for entry_index in 0..*count {
                if entries[entry_index as usize] == value {
                    if entry_index < *count {
                        entries[entry_index as usize] = entries[(*count - 1) as usize];
                    }
                    *count-=1;
                    if *count == 0 {
                        return false;
                    }
                    if !is_init {
                        update_guesses(&mut game.unknowns, row, column, *count);
                    }
                    break;
                }
            }
        }
    }

    // Update grid
    for target_column in ((column / 3) * 3) .. (((column/ 3) * 3) + 3) {
        for target_row in ((row / 3) * 3) .. (((row / 3) * 3) + 3) {
            if (target_column != column) && (target_row != row) {
                let (ref mut count, ref mut entries) = &mut game.board[target_row][target_column];
                for entry_index in 0..*count {
                    if entries[entry_index as usize] == value {
                        if entry_index < *count {
                            entries[entry_index as usize] = entries[(*count - 1) as usize];
                        }
                        *count-=1;
                        if *count == 0 {
                            return false;
                        }
                        if !is_init {
                            update_guesses(&mut game.unknowns, row, column, *count);
                        }
                        break;
                    }
                }
            }
        }
    }

    return true;
}

fn solve(game_state : &mut GameState ) -> bool {
    let mut remaining_count : u8 = 2;
    while remaining_count < 10 {
        // Check to see we have any that need guessing
        if let Some((row_to_guess, column_to_guess)) = game_state.unknowns.get_mut(&remaining_count).unwrap().pop() {
            // Iterate over all guesses
            let (count, guesses) = game_state.board[row_to_guess as usize][column_to_guess as usize];
            for current_count in 0..count {
                // Backup board
                let backup_unknowns = game_state.unknowns.clone();
                let backup_board = game_state.board.clone();

                let guess = guesses[current_count as usize];
                game_state.board[row_to_guess as usize][column_to_guess as usize].0 -= 1;
                println!("Guessing {} for ({}, {})", guess, row_to_guess+1, column_to_guess+1);
                if !update(game_state, row_to_guess as usize, column_to_guess as usize, guess, false) {
                    println!("Obviously bad guess. Failed on update");
                    game_state.unknowns = backup_unknowns;
                    game_state.board = backup_board;
                } else {
                    if !solve(game_state) {
                        game_state.unknowns = backup_unknowns;
                        game_state.board = backup_board;
                    } else {
                        println!("Winner winner");
                        return true;
                    }
                }
            }
            return false;
        }
        else {
            remaining_count += 1;
        }
    }
    return true;
}

fn main() {
    // https://doc.sagemath.org/html/en/reference/games/sage/games/sudoku.html
    let game = "5...8..49...5...3..673....115..........2.8..........187....415..3...2...49..5...3";
    //let game = "51368724.8495216.72673495811584639729..21.........54.87.2934156635172894491856723";
    let mut game_state = initialize_state(&game[0..]);
    solve(&mut game_state);
    print_state(&game_state);
}