use sfml::{
    graphics::{
        Color, RectangleShape, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
};
use rand::Rng;
use clap::{Arg, App};
use std::time::Instant;

#[derive(Copy, Clone)]
struct Spore {
    alive: bool,
    dietime: isize
}

fn main() {
    // get command line options
    let clapapp = App::new("Conways")
        .version("0.0.1")
        .about("creates a graphical 'conways game of life' game")
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .value_name("BOARDWIDTH")
            .about("sets the width of the board")
            .default_value("75")
            .takes_value(true))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .value_name("BOARDHEIGHT")
            .about("sets the height of the board")
            .default_value("50")
            .takes_value(true))
        .arg(Arg::new("scale")
            .short('s')
            .long("scale")
            .value_name("BOARDSCALE")
            .about("increases the size of each spore in the window")
            .default_value("10")
            .takes_value(true))
        .arg(Arg::new("randomspores")
            .short('r')
            .long("randomspores")
            .value_name("RANDOMSPORES")
            .about("the amount of random spores to spawn at startup")
            .default_value("500")
            .takes_value(true))
        .arg(Arg::new("consoleonly")
            .short('c')
            .long("consoleonly")
            .value_name("CONSOLEONLY")
            .about("do not create window and only output to console")
            .takes_value(false))
        .get_matches();

    let boardwidth: usize = clapapp.value_of_t("width").unwrap();
    let boardheight: usize = clapapp.value_of_t("height").unwrap();
    let boardscale: usize = clapapp.value_of_t("scale").unwrap();
    let randomspores: usize = clapapp.value_of_t("randomspores").unwrap();
    let consoleonly: bool = clapapp.is_present("consoleonly");

    // creates the board
    let mut board = vec![vec![Spore {alive: false, dietime: -1}; boardwidth]; boardheight];
    let mut rng = rand::thread_rng();

    // creates random spores
    for _ in 0..randomspores { 
        board[rng.gen_range(1..(boardheight-1))][rng.gen_range(1..(boardwidth-1))] = Spore {alive: true, dietime: 0};
    }
    
    // initalize the window
    let mut window = RenderWindow::new(
        ((boardwidth * boardscale) as u32, (boardheight * boardscale) as u32),
        "Conways",
        Style::CLOSE,
        &ContextSettings::default()
    );

    // start the running loop
    let mut frame: usize = 0;

    let mut block = RectangleShape::new();
    block.set_fill_color(Color::BLACK);
    block.set_size(Vector2f::new(boardscale as f32, boardscale as f32));

    loop {
        // print board to console
        if consoleonly {
            print_board(&board, frame, boardwidth);
        } else {
            // update title with frame
            window.set_title(&format!("Conways, Frame: {}", frame));

            // draw board to window
            window.clear(Color::WHITE);

            for i in 0..boardheight {
                for j in 0..boardwidth {
                    if board[i][j].alive == true {
                        block.set_position(Vector2f::new((j * boardscale) as f32, (i * boardscale) as f32));
                        window.draw(&block.clone());
                    }
                }
            }

            window.display();
        }

        // check for key presses, partly copied from rust-sfml docs
        while let Some(event) = window.poll_event() {
            // if Escape is pressed close the window
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
            // if Space if pressed update the board 1 frame
            match event {
                Event::KeyPressed {
                    code: Key::Space, ..
                } => {
                    let start = Instant::now();
                    board = update_board(&board, frame, boardwidth, boardheight);
                    frame+=1;
                    let duration = start.elapsed();
                    println!("Took {:?} to update frame {}", duration, frame);
                },
                _ => {}
            }
        }
    }
}

fn print_board(board: &Vec<Vec<Spore>>, frame: usize, width: usize) {
    let lines = String::from("_".repeat(width*2));
    println!("Frame: {}", frame);
    println!("{}", lines);
    for column in board.iter() {
        let mut outline = String::new();
        for element in column.iter() {
            if element.alive == true {
                outline.push_str("O ");
            } else {
                outline.push_str(". ");
            }
        }
        println!("{}", outline);
    }
    println!("{}", lines);
}

fn update_board(board: &Vec<Vec<Spore>>, frame: usize, width: usize, height: usize) -> Vec<Vec<Spore>> {
    let mut outboard = vec![vec![Spore {alive: false, dietime: -1}; width]; height];

    for i in 0..height {
        for j in 0..width {
            let neighbours: usize = get_neighbours(board, j, i, width, height);

            if board[i][j].alive == true {
                // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                if neighbours < 2 {
                    outboard[i][j].alive = false;
                    outboard[i][j].dietime = frame as isize;
                }

                // 2. Any live cell with two or three live neighbours lives on to the next generation.
                if neighbours == 2 || neighbours == 3 {
                    outboard[i][j] = board[i][j];
                }

                // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                if neighbours > 3 {
                    outboard[i][j].alive = false;
                    outboard[i][j].dietime = frame as isize;
                }
            } else {
                // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                if neighbours == 3 {
                    outboard[i][j].alive = true;
                    outboard[i][j].dietime = -1;
                }
            }
        }
    }

    return outboard;
}

fn get_neighbours(board: &Vec<Vec<Spore>>, x: usize, y: usize, width: usize, height: usize) -> usize {
    let mut neighbours: usize = 0;
    //println!("x: {}, y: {}, neighbors: {}", x, y, neighbours);
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return 0;
    }

    if board[y+1][x-1].alive == true { // top right
        neighbours+=1;
    }

    if board[y][x-1].alive == true { // top
        neighbours+=1;
    }

    if board[y-1][x-1].alive == true { // top left
        neighbours+=1;
    }

    if board[y+1][x].alive == true { // right
        neighbours+=1;
    }

    if board[y-1][x].alive == true { // left
        neighbours+=1;
    }

    if board[y][x+1].alive == true { // bottom
        neighbours+=1;
    }

    if board[y+1][x+1].alive == true { // bottom right
        neighbours+=1;
    }

    if board[y-1][x+1].alive == true { // bottom left
        neighbours+=1;
    }

    return neighbours;
}