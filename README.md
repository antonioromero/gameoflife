# GameOfLife

## Motivation

This is a naive command-line implementation of the Conway's Game of Life. The motivation of this project is an interest in learning some basics of Rust while developing a simple game.

The "game" is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. More at [Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

## How to start

You have to compile the project with cargo
```bash
cargo build
```
and prepare a text file with a matrix of zeros(0) representing dead cells and ones (1) representing living cells.

```bash
0000000000
0111000010
1001100010
0010010000
0000000000
```


The path must be passed as an argument.

```bash
./gameoflife /tmp/board.txt
```

## Feedback

Please use [GitHub issues](https://github.com/antonioromero/gameoflife/issues)
if you want to comment some issue, make a recommendation or you have some question
about the code.
