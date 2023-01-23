use std::{io, cmp::max, process::exit, env};
use rand::Rng;

const N: usize = 3;
const DEPTH_SEARCH: i32 = 7;
const POSINFINITY: i32 = 1000;
const NEGINFINITY: i32 = -1000;

struct Board {
    tiles: [i32; N*N],
}

fn print_board(b: &Board) {
    println!();
    for i in 0..b.tiles.len() {
        print!("|");
        match b.tiles[i] {
            0 => print!(" 0 "),
            1 => print!(" X "),
            _ => print!("   ")
        }
        if i%3 == 2 {
            print!("|\n");
        }
    }
    println!();
}

fn other_player(player: i32) -> i32 {
    if player == 0 {return 1;}
    else {return 0;}
}

fn ask_move(bsize: i32) -> i32 {
    let mut fila = -1;
    let mut columna = -1;
    
    while fila == -1 {
        println!("Digues fila [0, 1, 2]");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                fila = match input.trim().parse::<i32>() {
                    Ok(fila) => fila,
                    Err(error) => {println!("No es un enter: {}", error); -1},
                }
            }
            Err(error) => println!("Error de lectura: {}", error),
        };

        if fila < 0 || fila >= bsize {
            fila = -1;
            println!("FILA INCORRECTA!");
        }
    }

    while columna == -1 {
        println!("Digues columna [0, 1, 2]");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                columna = match input.trim().parse::<i32>() {
                    Ok(columna) => columna,
                    Err(error) => {println!("No es un enter: {}", error); -1},
                }
            }
            Err(error) => println!("Error de lectura: {}", error),
        };

        if columna < 0 || columna >= bsize {
            columna = -1;
            println!("COLUMNA INCORRECTA!");
        }
    }

    return fila*3 + columna;
}

//Retorna l'evaluacio del millor moviment
fn eval_move(b: &mut Board, player: i32, depth: i32) -> i32 {
    if check_winner(b, player) {return POSINFINITY;}
    else if check_winner(b, other_player(player)) {return NEGINFINITY;}
    else if check_draw(b) { return -10;}
    else if depth <= 0 {return 0;}
    
    let mut max_eval = NEGINFINITY;
    for i in 0..N*N {
        if b.tiles[i] != -1 {continue;}

        b.tiles[i] = player;
        let eval = -eval_move(b, other_player(player), depth-1);
        max_eval = max(max_eval, eval);
        b.tiles[i] = -1;
    }

    return max_eval;
}

//Retorna el moviment que s'ha de fer. Falla si hi ha dos moviments guanyadors
fn ai_move(b: &mut Board, player: i32) -> i32 {
    let mut max_eval = i32::MIN;
    let mut max_pos = 0;
    for i in 0..N*N {
        if b.tiles[i] != -1 {continue;}

        b.tiles[i] = player;
        let eval = -eval_move(b, other_player(player), DEPTH_SEARCH);
        let pos = i as i32;
        //println!("POS: {}, EVAL: {}", pos, eval);
        if eval > max_eval {
            max_eval = eval;
            max_pos = pos;
        }
        b.tiles[i] = -1;
    }
    return max_pos;
}

fn check_draw(b: &Board) -> bool {
    for i in 0..N*N {
        if b.tiles[i] == -1 { return false; }
    }
    return true;
}

fn check_winner(b: &Board, player: i32) -> bool {
    //Check rows & columns
    for i in 0..N {
        let mut winner_rows = true;
        let mut winner_columns = true;
        for j in 0..N {
            if b.tiles[3*i+j] != player {winner_rows = false;}
            if b.tiles[3*j+i] != player {winner_columns = false;}
        }
        if winner_rows||winner_columns {return true;}
    }

    //Check diagonals
    let mut winner_diagonal_1 = true;
    let mut winner_diagonal_2 = true;

    for i in 0..N {
        if b.tiles[3*i+i] != player  {winner_diagonal_1 = false;}
        if b.tiles[3*i+(N-i-1)] != player  {winner_diagonal_2 = false;}
    }
    return winner_diagonal_1||winner_diagonal_2;
}

fn usage() {
    println!("./tictactoe_rust P0 PX");
    println!("0 -> huma");
    println!("1 -> IA");
    println!("2 -> IA random");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {usage();}
    
    let player_0: i32 = args[1].trim().parse::<i32>().expect("Jugador 0 no es un enter");
    let player_x: i32 = args[2].trim().parse::<i32>().expect("Jugador X no es un enter");

    if !(0..3).contains(&player_0) {usage();}
    if !(0..3).contains(&player_x) {usage();}

    println!("COMENCEM!");

    let mut board = Board{
        tiles: [-1; (N*N)],
    };
    let mut player = 0;
    let mut game_end = false;
    let mut rng = rand::thread_rng();

    print_board(&board);

    while !game_end {
        let player_ai;
        if player == 0 {
            println!("JUGA 0");
            player_ai = player_0;
        } else {
            println!("JUGA X");
            player_ai = player_x;
        }

        let mut moviment;
        match player_ai {
            0 => {
                moviment = ask_move(N as i32) as usize;
                while board.tiles[moviment] != -1 {
                    println!("CASELLA OCUPADA!");
                    moviment = ask_move(N as i32) as usize;
                }
            },
            1 => {
                moviment = ai_move(&mut board, player) as usize;
                println!("LA IA MOU A {}", moviment);
            },
            _ => {
                moviment = rng.gen_range(0..9);
                while board.tiles[moviment] != -1 {
                    moviment = rng.gen_range(0..9);
                }
                println!("LA IA MOU A {}", moviment);
            }
        }
        board.tiles[moviment] = player;

        //Comprovem si s'ha acabat la partida
        if player == 0 && check_winner(&board, 0) {game_end = true; println!("GUANYA 0!");}
        else if player == 1 && check_winner(&board, 1) {game_end = true; println!("GUANYA X!");}
        else if check_draw(&board) {game_end = true; println!("EMPAT!");}
                
        player = other_player(player);
        print_board(&board);
    }
}
