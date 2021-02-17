# Conways
Conway's game of life in rust with a graphical frontend. Read about the rules [here](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). Press space to advance frame.

![framegif](https://raw.githubusercontent.com/mgurga/conways/master/docs/window.gif)

## Usage
```
USAGE:
    conways [FLAGS] [OPTIONS]

FLAGS:
    -c, --consoleonly    do not create window and only output to console
        --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -h, --height <BOARDHEIGHT>           sets the height of the board [default: 50]
    -r, --randomspores <RANDOMSPORES>
            the amount of random spores to spawn at startup [default: 500]

    -s, --scale <BOARDSCALE>
            increases the size of each spore in the window [default: 10]

    -w, --width <BOARDWIDTH>             sets the width of the board [default: 75]
```

## Screenshots
![graphical window](https://raw.githubusercontent.com/mgurga/conways/master/docs/window.png)
![console output](https://raw.githubusercontent.com/mgurga/conways/master/docs/console.png)
