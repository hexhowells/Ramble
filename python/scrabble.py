from collections import Counter
import random


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

    def __init__(
            self, 
            player1: Player, 
            player2: Player
        ):
        self.board = [
            ['T','#','#','d','#','#','#','T','#','#','#','d','#','#','T'],
            ['#','D','#','#','#','t','#','#','#','t','#','#','#','D','#'],
            ['#','#','D','#','#','#','d','#','d','#','#','#','D','#','#'],
            ['d','#','#','D','#','#','#','d','#','#','#','D','#','#','d'],
            ['#','#','#','#','D','#','#','#','#','#','D','#','#','#','#'],
            ['#','t','#','#','#','t','#','#','#','t','#','#','#','t','#'],
            ['#','#','d','#','#','#','d','#','d','#','#','#','d','#','#'],
            ['T','#','#','d','#','#','#','#','#','#','#','d','#','#','T'],
            ['#','#','d','#','#','#','d','#','d','#','#','#','d','#','#'],
            ['#','t','#','#','#','t','#','#','#','t','#','#','#','t','#'],
            ['#','#','#','#','D','#','#','#','#','#','D','#','#','#','#'],
            ['d','#','#','D','#','#','#','d','#','#','#','D','#','#','d'],
            ['#','#','D','#','#','#','d','#','d','#','#','#','D','#','#'],
            ['#','D','#','#','#','t','#','#','#','t','#','#','#','D','#'],
            ['T','#','#','d','#','#','#','T','#','#','#','d','#','#','T'],
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
        return True


    def play(self, player: Player, moves: list[tuple[str, tuple]]):
        if self.first_turn:
            if (7, 7) not in [m[1] for m in moves]:
                print("First turn needs to play through center square")
                return
        
        for move in moves:
            tile, coords = move
            r, c = coords
            if self.board[r][c] != '#':
                print(f"Square ({r},{c}) already contains a tile")
                return
            self.board[r][c] = tile

        if self.validate():
            for move in moves:
                tile, _ = move
                player.rack[tile] -= 1

            self.draw(player, len(moves))
            
            if self.first_turn:
                self.first_turn = False
        else:
            for move in moves:
                tile, coords = move
                r, c = coords
                self.board[r][c] = '#'


if __name__ == "__main__":
    p1 = Player()
    p2 = Player()
    board = Scrabble(p1, p2)

    board.state()

    t1, t2, t3 = p1.rack_tiles[:3]
    moves = [(t1, (7,7)), (t2, (7,8)), (t3, (7,9))]
    board.play(p1, moves)

    board.state()