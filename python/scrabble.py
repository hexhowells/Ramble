from collections import Counter
import random
from enum import Enum


class Multiplier(Enum):
    DOUBLE_LETTER = 'd'
    TRIPLE_LETTER = 't'
    DOUBLE_WORD = 'D'
    TRIPLE_WORD = 'T'


class Player:
    def __init__(self):
        self.score: int = 0
        self.rack = Counter({
            'A': 0, 'B': 0, 'C': 0, 'D': 0, 'E': 0, 'F': 0, 'G': 0, 'H': 0,
            'I': 0, 'J': 0, 'K': 0, 'L': 0, 'M': 0, 'N': 0, 'O': 0, 'P': 0,
            'Q': 0, 'R': 0, 'S': 0, 'T': 0, 'U': 0, 'V': 0, 'W': 0, 'X': 0,
            'Y': 0, 'Z': 0, '[Blank]': 0
        })


    @property
    def rack_tiles(self):
        return list(self.rack.elements())


class Scrabble:
    tile_scores = {
        'A': 1, 'B': 3, 'C': 3, 'D': 2, 'E': 1, 'F': 4, 'G': 2, 'H': 4,
        'I': 1, 'J': 8, 'K': 5, 'L': 1, 'M': 3, 'N': 1, 'O': 1, 'P': 3,
        'Q': 10, 'R': 1, 'S': 1, 'T': 1, 'U': 1, 'V': 4, 'W': 4, 'X': 8,
        'Y': 4, 'Z': 10, '[Blank]': 0
    }

    multipliers: dict[tuple[int, int], Multiplier] = {
        (0, 0): Multiplier.TRIPLE_WORD, (0, 7): Multiplier.TRIPLE_WORD, 
        (0, 14): Multiplier.TRIPLE_WORD, (7, 0): Multiplier.TRIPLE_WORD, 
        (7, 14): Multiplier.TRIPLE_WORD, (14, 0): Multiplier.TRIPLE_WORD, 
        (14, 7): Multiplier.TRIPLE_WORD, (14, 14): Multiplier.TRIPLE_WORD,
        (1, 1): Multiplier.DOUBLE_WORD, (1, 13): Multiplier.DOUBLE_WORD,
        (2, 2): Multiplier.DOUBLE_WORD, (2, 12): Multiplier.DOUBLE_WORD,
        (3, 3): Multiplier.DOUBLE_WORD, (3, 11): Multiplier.DOUBLE_WORD,
        (4, 4): Multiplier.DOUBLE_WORD, (4, 10): Multiplier.DOUBLE_WORD,
        (10, 4): Multiplier.DOUBLE_WORD, (10, 10): Multiplier.DOUBLE_WORD,
        (11, 3): Multiplier.DOUBLE_WORD, (11, 11): Multiplier.DOUBLE_WORD,
        (12, 2): Multiplier.DOUBLE_WORD, (12, 12): Multiplier.DOUBLE_WORD,
        (13, 1): Multiplier.DOUBLE_WORD, (13, 13): Multiplier.DOUBLE_WORD,
        (1, 5): Multiplier.TRIPLE_LETTER, (1, 9): Multiplier.TRIPLE_LETTER,
        (5, 1): Multiplier.TRIPLE_LETTER, (5, 5): Multiplier.TRIPLE_LETTER, 
        (5, 9): Multiplier.TRIPLE_LETTER, (5, 13): Multiplier.TRIPLE_LETTER,
        (9, 1): Multiplier.TRIPLE_LETTER, (9, 5): Multiplier.TRIPLE_LETTER, 
        (9, 9): Multiplier.TRIPLE_LETTER, (9, 13): Multiplier.TRIPLE_LETTER,
        (13, 5): Multiplier.TRIPLE_LETTER, (13, 9): Multiplier.TRIPLE_LETTER,
        (0, 3): Multiplier.DOUBLE_LETTER, (0, 11): Multiplier.DOUBLE_LETTER,
        (2, 6): Multiplier.DOUBLE_LETTER, (2, 8): Multiplier.DOUBLE_LETTER,
        (3, 0): Multiplier.DOUBLE_LETTER, (3, 7): Multiplier.DOUBLE_LETTER, 
        (3, 14): Multiplier.DOUBLE_LETTER, (6, 2): Multiplier.DOUBLE_LETTER, 
        (6, 6): Multiplier.DOUBLE_LETTER, (6, 8): Multiplier.DOUBLE_LETTER, 
        (6, 12): Multiplier.DOUBLE_LETTER, (7, 3): Multiplier.DOUBLE_LETTER, 
        (7, 11): Multiplier.DOUBLE_LETTER, (8, 2): Multiplier.DOUBLE_LETTER, 
        (8, 6): Multiplier.DOUBLE_LETTER, (8, 8): Multiplier.DOUBLE_LETTER, 
        (8, 12): Multiplier.DOUBLE_LETTER, (11, 0): Multiplier.DOUBLE_LETTER, 
        (11, 7): Multiplier.DOUBLE_LETTER, (11, 14): Multiplier.DOUBLE_LETTER, 
        (12, 6): Multiplier.DOUBLE_LETTER, (12, 8): Multiplier.DOUBLE_LETTER, 
        (14, 3): Multiplier.DOUBLE_LETTER, (14, 11): Multiplier.DOUBLE_LETTER
    }

    def __init__(
            self, 
            player1: Player, 
            player2: Player
        ):
        self.board = [
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
            ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
        ]
        self.first_turn = True
        self.player1 = player1
        self.player2 = player2
        self.bag = Counter({
            'A': 9, 'B': 2, 'C': 2, 'D': 4, 'E': 12, 'F': 2, 'G': 3, 'H': 2,
            'I': 9, 'J': 1, 'K': 1, 'L': 4, 'M': 2, 'N': 5, 'O': 8, 'P': 2,
            'Q': 1, 'R': 6, 'S': 4, 'T': 6, 'U': 4, 'V': 2, 'W': 2, 'X': 1,
            'Y': 2, 'Z': 1, '[Blank]': 2
        })
        self.draw(self.player1, 7)
        self.draw(self.player2, 7)


    def __str__(self) -> str:
        return '\n'.join([' '.join(row) for row in self.board])


    def state(self) -> None:
        print('-'*100)
        print(self)
        print(f'\n(Player 1) - Score: {self.player1.score} - Rack: {self.player1.rack_tiles}')
        print(f'(Player 2) - Score: {self.player2.score} - Rack: {self.player2.rack_tiles}')
        print(f'\nBag: {self.bag}')


    def draw(self, player: Player, n: int):
        tiles = list(self.bag.elements())

        assert player.rack.total() + n <= 7, f"Cannot draw {n} tiles, rack cannot have more than 7 tiles."

        n = min(n, len(tiles))

        sampled = random.sample(tiles, n)

        for tile in sampled:
            self.bag[tile] -= 1
            player.rack[tile] += 1


    def exchange(self, player: Player, tiles: list[str]) -> None:
        if len(self.bag) == 0:
            return None

        if len(tiles) > len(self.bag):
            return None

        assert len(set(tiles) - set(player.rack)) == 0, "Cannot exchange tiles that are not in the player's rack."

        for tile in tiles:
            player.rack[tile] -= 1

        self.draw(player, len(tiles))

        for tile in tiles:
            self.bag[tile] += 1


    def validate(self):
        """
        check if first and last tile are on X or Y axis (if neither then diag = invalid)
        step from first tile to last to ensure all tiles are present (no gaps, no tiles elsewhere)
        check tiles touch at least one existing tile
        check word is valid (can build the word from the previous step, O(1) dict lookup)
        """
        return True


    def update_score(self, player, moves):
        score = 0
        dw = 0
        tw = 0

        for move in moves:
            tile, coords = move
            if coords in self.multipliers:
                match self.multipliers[coords]:
                    case Multiplier.DOUBLE_LETTER:
                        score += self.tile_scores[tile] * 2
                    case Multiplier.TRIPLE_LETTER:
                        score += self.tile_scores[tile] * 3
                    case Multiplier.DOUBLE_WORD:
                        dw += 1
                    case Multiplier.TRIPLE_WORD:
                        tw += 1
            else:
                score += self.tile_scores[tile]

        score = score * 2**dw
        score = score * 3**tw

        player.score += score


    def play(self, player: Player, moves: list[tuple[str, tuple]]) -> bool:
        if self.first_turn:
            if (7, 7) not in [m[1] for m in moves]:
                print("First turn needs to play through center square")
                return False
        
        for move in moves:
            tile, coords = move
            r, c = coords
            if self.board[r][c] != '#':
                print(f"Square ({r},{c}) already contains a tile")
                return False
            self.board[r][c] = tile

        if self.validate():
            for move in moves:
                tile, _ = move
                player.rack[tile] -= 1

            self.draw(player, len(moves))
            
            if self.first_turn:
                self.first_turn = False

            self.update_score(player, moves)
            
            return True
        
        for move in moves:
            tile, coords = move
            r, c = coords
            self.board[r][c] = '#'
        return False


if __name__ == "__main__":
    p1 = Player()
    p2 = Player()
    board = Scrabble(p1, p2)

    board.state()

    t1, t2, t3 = p1.rack_tiles[:3]
    moves = [(t1, (7,7)), (t2, (7,8)), (t3, (7,9))]
    board.play(p1, moves)

    board.state()