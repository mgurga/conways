use sfml::{
    graphics::{
        Color, RectangleShape, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
};
use rand::Rng;

const BOARDWIDTH: usize = 75;      // the width of the board
const BOARDHEIGHT: usize = 50;     // the height of the board
const BOARDSCALE: usize = 10;      // how large the board should be scaled in the window
const RANDOMSPORES: usize = 500;   // the amount of random spores on the board at startup
const CONSOLEOUTPUT: bool = false; // print the board to the terminal

#[derive(Copy, Clone)]
struct Spore {
    alive: bool,
    dietime: isize
}

fn main() {
    // creates the board
    let mut board = [[Spore {alive: false, dietime: -1}; BOARDWIDTH]; BOARDHEIGHT];
    let mut rng = rand::thread_rng();

    // creates random spores
    for _ in 0..RANDOMSPORES { 
        board[rng.gen_range(1..(BOARDHEIGHT-1))][rng.gen_range(1..(BOARDWIDTH-1))] = Spore {alive: true, dietime: 0};
    }

    // initalize the window
    let mut window = RenderWindow::new(
        ((BOARDWIDTH * BOARDSCALE) as u32, (BOARDHEIGHT * BOARDSCALE) as u32),
        "Conways",
        Style::CLOSE,
        &ContextSettings::default()
    );

    // start the running loop
    let mut frame: usize = 0;

    let mut block = RectangleShape::new();
    block.set_fill_color(Color::BLACK);
    block.set_size(Vector2f::new(BOARDSCALE as f32, BOARDSCALE as f32));

    loop {
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
                    board = update_board(&board, frame);
                    frame+=1;
                },
                _ => {}
            }
        }

        // update title with frame
        window.set_title(&format!("Conways, Frame: {}", frame));

        // print board to console
        if CONSOLEOUTPUT {
            let lines = String::from("_".repeat(BOARDWIDTH*2));
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

        // draw board to window
        window.clear(Color::WHITE);
        for i in 0..BOARDHEIGHT {
            for j in 0..BOARDWIDTH {
                if board[i][j].alive == true {
                    block.set_position(Vector2f::new((j * BOARDSCALE) as f32, (i * BOARDSCALE) as f32));
                    window.draw(&block.clone());
                }
            }
        }
        window.display();
    }
}

fn update_board(board: &[[Spore; BOARDWIDTH]; BOARDHEIGHT], frame: usize) -> [[Spore; BOARDWIDTH]; BOARDHEIGHT] {
    let mut outboard = [[Spore {alive: false, dietime: -1}; BOARDWIDTH]; BOARDHEIGHT];

    for i in 0..BOARDHEIGHT {
        for j in 0..BOARDWIDTH {
            let neighbours: usize = get_neighbours(board, j, i);

            // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            if board[i][j].alive == true && neighbours < 2 {
                outboard[i][j].alive = false;
                outboard[i][j].dietime = frame as isize;
            }

            // 2. Any live cell with two or three live neighbours lives on to the next generation.
            if board[i][j].alive == true && (neighbours == 2 || neighbours == 3) {
                outboard[i][j] = board[i][j];
            }

            // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
            if board[i][j].alive == true && neighbours > 3 {
                outboard[i][j].alive = false;
                outboard[i][j].dietime = frame as isize;
            }

            // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            if board[i][j].alive == false && neighbours == 3 {
                outboard[i][j].alive = true;
                outboard[i][j].dietime = -1;
            }
        }
    }

    return outboard;
}

fn get_neighbours(board: &[[Spore; BOARDWIDTH]; BOARDHEIGHT], x: usize, y: usize) -> usize {
    let mut neighbours: usize = 0;
    //println!("x: {}, y: {}, neighbors: {}", x, y, neighbours);
    if x == 0 || y == 0 || x == BOARDWIDTH - 1 || y == BOARDHEIGHT - 1 {
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