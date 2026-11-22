# Ramble
A Scrabble engine written in Rust. [WIP]

Wrote a basic engine in python (no cpu opponent) first in `python/` because im a noob at rust.

## Usage
The engine has the following commands:
- `play` - to play a word
- `exchange` - to exchange tiles in the bag
- `quit` - to exit the program

You can play a word using basic scrabble notation, below is the basic play command:

`play <player_index> <word_location> <word>`

where the word location is the column and row indexes, where a horizonal word starts with the numerical index and a vertical word starts with the alphabetical index.

The below command plays the word "ramble" for the first player (0) starting at the tile `8H` in the horizontal direction:

`play 0 8H RAMBLE`

### Blank tiles
Blank tiles are represented in the player's rack as an `*` symbol. You can play the blank tile by using the `*` symbol, followed by the letter you want the blank tile to represent.

## Data
Number of words in NWL2020: 172,820

Number of words in CSW2019: 279,496
