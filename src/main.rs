use core::panic;
use std::process::Command;
use std::{io::{self, Write}, thread, time::Duration};

/*
00│01│02
──┼──┼──
10│11│12
──┼──┼──
20│21│22
*/

struct SmallGrid {
    smallgrid: [[String; 3]; 3],
}

impl SmallGrid {
    fn new() -> SmallGrid {
        SmallGrid { 
            smallgrid: [
                [String::from(" "), String::from(" "), String::from(" ")],
                [String::from(" "), String::from(" "), String::from(" ")],
                [String::from(" "), String::from(" "), String::from(" ")],
            ],
        }
    }
}

struct BigGrid {
    biggrid: [[SmallGrid; 3]; 3],
    winning: [[String; 3]; 3]
}

impl BigGrid {
    fn new() -> BigGrid {
        BigGrid {
            biggrid: [
                [SmallGrid::new(), SmallGrid::new(), SmallGrid::new()],
                [SmallGrid::new(), SmallGrid::new(), SmallGrid::new()],
                [SmallGrid::new(), SmallGrid::new(), SmallGrid::new()],
            ],
            winning: [
                [String::from(" "), String::from(" "), String::from(" ")],
                [String::from(" "), String::from(" "), String::from(" ")],
                [String::from(" "), String::from(" "), String::from(" ")],
            ]
        }
    }
}

enum Player {
    One,
    Two,
}

struct PlayerTurn {
    player: Player,
}

impl PlayerTurn {
    // Method to toggle the player
    fn toggle_player(&mut self) {
        self.player = match self.player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }

    // Method to get the marker for the current player
    fn get_marker(&self) -> &str {
        match self.player {
            Player::One => "X",
            Player::Two => "O",
        }
    }
}

#[derive(PartialEq)]
enum CardinalDirection{
    N, NE, E, SE, S, SW, W, NW, C, All
}


fn print_playingfield(game: &BigGrid) {
    println!("             │             │");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ", 
    game.biggrid[0][0].smallgrid[0][0],game.biggrid[0][0].smallgrid[0][1],game.biggrid[0][0].smallgrid[0][2],
    game.biggrid[0][1].smallgrid[0][0],game.biggrid[0][1].smallgrid[0][1],game.biggrid[0][1].smallgrid[0][2],
    game.biggrid[0][2].smallgrid[0][0],game.biggrid[0][2].smallgrid[0][1],game.biggrid[0][2].smallgrid[0][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[0][0].smallgrid[1][0],game.biggrid[0][0].smallgrid[1][1],game.biggrid[0][0].smallgrid[1][2],
    game.biggrid[0][1].smallgrid[1][0],game.biggrid[0][1].smallgrid[1][1],game.biggrid[0][1].smallgrid[1][2],
    game.biggrid[0][2].smallgrid[1][0],game.biggrid[0][2].smallgrid[1][1],game.biggrid[0][2].smallgrid[1][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[0][0].smallgrid[2][0],game.biggrid[0][0].smallgrid[2][1],game.biggrid[0][0].smallgrid[2][2],
    game.biggrid[0][1].smallgrid[2][0],game.biggrid[0][1].smallgrid[2][1],game.biggrid[0][1].smallgrid[2][2],
    game.biggrid[0][2].smallgrid[2][0],game.biggrid[0][2].smallgrid[2][1],game.biggrid[0][2].smallgrid[2][2]);
    println!("             │             │");
    println!(" ────────────┼─────────────┼────────────");
    println!("             │             │");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[1][0].smallgrid[0][0],game.biggrid[1][0].smallgrid[0][1],game.biggrid[1][0].smallgrid[0][2],
    game.biggrid[1][1].smallgrid[0][0],game.biggrid[1][1].smallgrid[0][1],game.biggrid[1][1].smallgrid[0][2],
    game.biggrid[1][2].smallgrid[0][0],game.biggrid[1][2].smallgrid[0][1],game.biggrid[1][2].smallgrid[0][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[1][0].smallgrid[1][0],game.biggrid[1][0].smallgrid[1][1],game.biggrid[1][0].smallgrid[1][2],
    game.biggrid[1][1].smallgrid[1][0],game.biggrid[1][1].smallgrid[1][1],game.biggrid[1][1].smallgrid[1][2],
    game.biggrid[1][2].smallgrid[1][0],game.biggrid[1][2].smallgrid[1][1],game.biggrid[1][2].smallgrid[1][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[1][0].smallgrid[2][0],game.biggrid[1][0].smallgrid[2][1],game.biggrid[1][0].smallgrid[2][2],
    game.biggrid[1][1].smallgrid[2][0],game.biggrid[1][1].smallgrid[2][1],game.biggrid[1][1].smallgrid[2][2],
    game.biggrid[1][2].smallgrid[2][0],game.biggrid[1][2].smallgrid[2][1],game.biggrid[1][2].smallgrid[2][2]);
    println!("             │             │");
    println!(" ────────────┼─────────────┼────────────");
    println!("             │             │");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[2][0].smallgrid[0][0],game.biggrid[2][0].smallgrid[0][1],game.biggrid[2][0].smallgrid[0][2],
    game.biggrid[2][1].smallgrid[0][0],game.biggrid[2][1].smallgrid[0][1],game.biggrid[2][1].smallgrid[0][2],
    game.biggrid[2][2].smallgrid[0][0],game.biggrid[2][2].smallgrid[0][1],game.biggrid[2][2].smallgrid[0][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[2][0].smallgrid[1][0],game.biggrid[2][0].smallgrid[1][1],game.biggrid[2][0].smallgrid[1][2],
    game.biggrid[2][1].smallgrid[1][0],game.biggrid[2][1].smallgrid[1][1],game.biggrid[2][1].smallgrid[1][2],
    game.biggrid[2][2].smallgrid[1][0],game.biggrid[2][2].smallgrid[1][1],game.biggrid[2][2].smallgrid[1][2]);
    println!(" ───┼───┼─── │ ───┼───┼─── │ ───┼───┼───");
    println!("  {} │ {} │ {}  │  {} │ {} │ {}  │  {} │ {} │ {} ",
    game.biggrid[2][0].smallgrid[2][0],game.biggrid[2][0].smallgrid[2][1],game.biggrid[2][0].smallgrid[2][2],
    game.biggrid[2][1].smallgrid[2][0],game.biggrid[2][1].smallgrid[2][1],game.biggrid[2][1].smallgrid[2][2],
    game.biggrid[2][2].smallgrid[2][0],game.biggrid[2][2].smallgrid[2][1],game.biggrid[2][2].smallgrid[2][2]);
    println!("             │             │");
}

fn print_with_delay(text: &str, delay: Duration) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for ch in text.chars() {
        handle.write_all(ch.encode_utf8(&mut [0; 4]).as_bytes()).unwrap();
        handle.flush().unwrap();
        thread::sleep(delay);
    }

    println!(); // Move to the next line after the text is printed
}

fn startup_script() {
    let text = r#"
     _   _ _    _____ ________  ___  ___ _____ _____   _____ _____ _____  _   __  _____ ___  _____  _   __  _____ _____ _____ 
    | | | | |  |_   _|_   _|  \/  | / _ \_   _|  ___| |_   _|_   _/  __ \| | / / |_   _/ _ \/  __ \| | / / |_   _|  _  |  ___|
    | | | | |    | |   | | | .  . |/ /_\ \| | | |__     | |   | | | /  \/| |/ /    | |/ /_\ \ /  \/| |/ /    | | | | | | |__  
    | | | | |    | |   | | | |\/| ||  _  || | |  __|    | |   | | | |    |    \    | ||  _  | |    |    \    | | | | | |  __| 
    | |_| | |____| |  _| |_| |  | || | | || | | |___    | |  _| |_| \__/\| |\  \   | || | | | \__/\| |\  \   | | \ \_/ / |___ 
     \___/\_____/\_/  \___/\_|  |_/\_| |_/\_/ \____/    \_/  \___/ \____/\_| \_/   \_/\_| |_/\____/\_| \_/   \_/  \___/\____/ 
    "#;
    let delay = Duration::from_micros(1);
    print_with_delay(text, delay);
}

fn help_text() {
    println!("
 NW │ N │ NE        To adresse a playingfield use the cardinal directions as shown to the left.
 ───┼───┼───        When every field is available to be played on use the same cardinal directions for the \"big grid\" and then the \"small grid\":
  W │ C │ E         
 ───┼───┼───        
 SW │ S │ SE        

");
}

fn clear_console() {
    Command::new("cmd")
    .args(&["/C", "cls"])
    .status()
    .unwrap();
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned().to_lowercase()
}

fn mark_grid(grid: &mut BigGrid, direction: &CardinalDirection, filler: String) {
    //clear all fields
    for small_grid_row in &mut grid.biggrid {
        for small_grid in small_grid_row {
            for row in &mut small_grid.smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = " ".to_string();
                    }
                }
            }
        }
    }
    //fill whatever is selected
    match direction {
        CardinalDirection::NW => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[0][0].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::N => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[0][1].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::NE => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[0][2].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::W => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[1][0].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::C => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[1][1].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::E => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[1][2].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::SW => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[2][0].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::S => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[2][1].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::SE => {
            // Editing only biggrid[0][1] and its small grids
            for row in &mut grid.biggrid[2][2].smallgrid {
                for element in row {
                    if *element != "X" && *element != "O" && *element != "." {
                        *element = filler.clone();
                    }
                }
            }
        }
        CardinalDirection::All => {
            for small_grid_row in &mut grid.biggrid {
                for small_grid in small_grid_row {
                    for row in &mut small_grid.smallgrid {
                        for element in row {
                            if *element != "X" && *element != "O" && *element != "." {
                                *element = filler.clone();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    clear_console();
    startup_script();
    loop {
        println!("To start the game input \"Start\"");
        match user_input().as_str() {
            "start" => start_game(),
            _ => continue,
        }
    }
    //clear_console();
}

fn start_game() {
    let mut playingfield = BigGrid::new();
    let mut global_selection = true;
    let mut player_selection: CardinalDirection;
    let mut last_selection = CardinalDirection::All;
    let mut current_turn = PlayerTurn {player: Player::One};

    loop{
        clear_console();
        help_text();
        mark_grid(&mut playingfield, &last_selection, "█".to_string());
        print_playingfield(&playingfield);
        let input = user_input();

        player_selection = match input.as_str() {
            "nw" => CardinalDirection::NW,
            "n" => CardinalDirection::N,
            "ne" => CardinalDirection::NE,
            "e" => CardinalDirection::E,
            "se" => CardinalDirection::SE,
            "s" => CardinalDirection::S,
            "sw" => CardinalDirection::SW,
            "w" => CardinalDirection::W,
            "c" => CardinalDirection::C,
            "cancel" => {
                if global_selection {
                    player_selection = CardinalDirection::All;
                    last_selection = CardinalDirection::All;
                    mark_grid(&mut playingfield, &player_selection, "█".to_string());
                }
                continue;
            }
            _ => {
                continue
            }
        };
        let (big_row, big_column) = get_coordiante(&player_selection);
        if last_selection == CardinalDirection::All {
            if playingfield.winning[big_row][big_column] == " ".to_string() {
                mark_grid(&mut playingfield, &player_selection, "█".to_string());
                last_selection = player_selection;
                continue;
            } else {
                continue;
            }
        }
        if !place_player_mark(&last_selection, &player_selection, &mut playingfield, &current_turn.get_marker().to_string()) {
            continue;
        }
        if check_small_winning(&last_selection, &mut playingfield, current_turn.get_marker().to_string()) {
            let (big_row, big_column) = get_coordiante(&last_selection);
            mark_grid(&mut playingfield, &last_selection, ".".to_string());
            playingfield.winning[big_row][big_column] = current_turn.get_marker().to_string();
            if check_big_winning(&mut playingfield, current_turn.get_marker().to_string()) {
                winning_screen();
            }
        }
        last_selection = player_selection;
        if already_won(&playingfield, &last_selection) {
            last_selection = CardinalDirection::All;
            global_selection = true;
        } else {
            global_selection = false;
        }
        current_turn.toggle_player();
    }
}

fn place_player_mark(last_selection:&CardinalDirection, player_selection:&CardinalDirection, playingfield: &mut BigGrid, marker:&String) -> bool {
    let (big_row, big_column) = get_coordiante(last_selection);
    let (small_row, small_column) = get_coordiante(player_selection);

    if playingfield.biggrid[big_row][big_column].smallgrid[small_row][small_column] == "█".to_string() {
        playingfield.biggrid[big_row][big_column].smallgrid[small_row][small_column] = marker.to_string();
        true
    } else {
        false
    }
}

fn get_coordiante(coordinate:&CardinalDirection) -> (usize, usize) {
    /*
    00│01│02
    ──┼──┼──
    10│11│12
    ──┼──┼──
    20│21│22
    */
    match coordinate {
        CardinalDirection::NW => (0, 0),
        CardinalDirection::N => (0, 1),
        CardinalDirection::NE => (0, 2),
        CardinalDirection::E => (1, 2),
        CardinalDirection::SE => (2, 2),
        CardinalDirection::S => (2, 1),
        CardinalDirection::SW => (2, 0),
        CardinalDirection::W => (1, 0),
        CardinalDirection::C => (1, 1),
        _ => panic!("Invalid coordinate")
    }
}

fn already_won(playingfield: &BigGrid, last_selection: &CardinalDirection) -> bool {
    let (row, column) = get_coordiante(last_selection);
    if playingfield.winning[row][column] != " ".to_string() {
        return true
    }
    return false
}

fn check_small_winning(last_selection:&CardinalDirection, playingfield: &mut BigGrid, marker:String) -> bool {
    let (big_row, big_column) = get_coordiante(last_selection);
    for row in 0..3 {
        if playingfield.biggrid[big_row][big_column].smallgrid[row][0] == marker && playingfield.biggrid[big_row][big_column].smallgrid[row][1] == marker && playingfield.biggrid[big_row][big_column].smallgrid[row][2] == marker {
            return true;
        }
    }

    // Check columns
    for col in 0..3 {
        if playingfield.biggrid[big_row][big_column].smallgrid[0][col] == marker && playingfield.biggrid[big_row][big_column].smallgrid[1][col] == marker && playingfield.biggrid[big_row][big_column].smallgrid[2][col] == marker {
            return true;
        }
    }

    // Check diagonals
    if playingfield.biggrid[big_row][big_column].smallgrid[0][0] == marker && playingfield.biggrid[big_row][big_column].smallgrid[1][1] == marker && playingfield.biggrid[big_row][big_column].smallgrid[2][2] == marker {
        return true;
    }
    if playingfield.biggrid[big_row][big_column].smallgrid[0][2] == marker && playingfield.biggrid[big_row][big_column].smallgrid[1][1] == marker && playingfield.biggrid[big_row][big_column].smallgrid[2][0] == marker {
        return true
    }
    return false
}

fn check_big_winning(playingfield: &mut BigGrid, marker:String) -> bool {
    for row in 0..3 {
        if playingfield.winning[row][0] == marker && playingfield.winning[row][1] == marker && playingfield.winning[row][2] == marker {
            return true;
        }
    }

    // Check columns
    for col in 0..3 {
        if playingfield.winning[0][col] == marker && playingfield.winning[1][col] == marker && playingfield.winning[2][col] == marker {
            return true;
        }
    }

    // Check diagonals
    if playingfield.winning[0][0] == marker && playingfield.winning[1][1] == marker && playingfield.winning[2][2] == marker {
        return true;
    }
    if playingfield.winning[0][2] == marker && playingfield.winning[1][1] == marker && playingfield.winning[2][0] == marker {
        return true;
    }

    false
}

fn winning_screen() {
    clear_console();
    println!("You Win!");
    user_input();
}