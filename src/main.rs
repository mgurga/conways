use rand::Rng;

const BOARDWIDTH: usize = 10;      // the width of the board
const BOARDHEIGHT: usize = 10;     // the height of the board
const BOARDSCALE: usize = 10;      // how large the board should be scaled in the window
const RANDOMSPORES: usize = 10;    // the amount of random spores on the board at startup

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
    for i in 0..RANDOMSPORES { 
        board[rng.gen_range(1..(BOARDHEIGHT-1))][rng.gen_range(1..(BOARDWIDTH-1))] = Spore {alive: true, dietime: 0};
    }

    // TODO: initalize the window

    // start the running loop
    let mut running = true;
    let mut frame: usize = 0;
    while running {

        // draw board
        // first print it
        let mut lines = String::new();
        lines = "_".repeat(BOARDWIDTH*2);
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

        // TODO: second draw to window

        // update board
        board = update_board(&board, frame);

        // next frame
        frame+=1;
    }
}

fn update_board(board: &[[Spore; BOARDWIDTH]; BOARDHEIGHT], frame: usize) -> [[Spore; BOARDWIDTH]; BOARDHEIGHT] {
    let mut outboard = [[Spore {alive: false, dietime: -1}; BOARDWIDTH]; BOARDHEIGHT];

    for i in 0..BOARDHEIGHT {
        for j in 0..BOARDWIDTH {
            let neighbours: usize = get_neighbours(board, i, j);

            // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            if board[i][j].alive == true && neighbours < 2 {
                outboard[i][j].alive = false;
                outboard[i][j].dietime = frame as isize;
            }

            // 2. Any live cell with two or three live neighbours lives on to the next generation.
            if board[i][j].alive == true && (neighbours == 2 || neighbours == 3) {
                // live on
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

    if x == 0 || y == 0 || x == BOARDWIDTH - 1 || y == BOARDHEIGHT - 1 {
        return 0;
    }

    if board[x+1][y-1].alive == true { // top right
        neighbours+=1;
    }

    if board[x][y-1].alive == true { // top
        neighbours+=1;
    }

    if board[x-1][y-1].alive == true { // top left
        neighbours+=1;
    }

    if board[x+1][y].alive == true { // right
        neighbours+=1;
    }

    if board[x-1][y].alive == true { // left
        neighbours+=1;
    }

    if board[x][y+1].alive == true { // bottom
        neighbours+=1;
    }

    if board[x+1][y+1].alive == true { // bottom right
        neighbours+=1;
    }

    if board[x-1][y+1].alive == true { // bottom left
        neighbours+=1;
    }

    return neighbours;
}