import tkinter as tk
from tkinter import messagebox, simpledialog
from collections import Counter
from scrabble import Player, Scrabble, Multiplier

class ScrabbleUI(tk.Tk):
    THEME = {
        'bg_main': '#2B2B2B',
        'bg_panel': '#3C3F41',
        'fg_text': '#EAEAEA',
        'board_bg': '#1E1E1E',
        'tile_placed': '#E8C396',
        'tile_pending': '#FFF5A5',
        'btn_submit': '#4CAF50',
        'btn_clear': '#F44336'
    }

    BOARD_COLORS = {
        'default': '#DFD4C5',
        Multiplier.TRIPLE_WORD: '#E57373',
        Multiplier.DOUBLE_WORD: '#FFB74D',
        Multiplier.TRIPLE_LETTER: '#4FC3F7',
        Multiplier.DOUBLE_LETTER: '#B2EBF2',
    }
    
    BOARD_LABELS = {
        Multiplier.TRIPLE_WORD: 'TW',
        Multiplier.DOUBLE_WORD: 'DW',
        Multiplier.TRIPLE_LETTER: 'TL',
        Multiplier.DOUBLE_LETTER: 'DL',
    }

    def __init__(self):
        super().__init__()
        self.title("Scrabble Engine Inspector")
        self.geometry("1100x850")
        self.configure(bg=self.THEME['bg_main'])
        
        self.p1 = Player()
        self.p2 = Player()
        self.game = Scrabble(self.p1, self.p2)
        
        self.current_player = self.p1
        self.current_player_name = "Player 1"
        self.selected_tile = None      
        self.pending_moves = []        
        self.placed_mask = [[False for _ in range(15)] for _ in range(15)]
        
        self._build_ui()
        self.refresh()

    def _build_ui(self):
        self.left_col = tk.Frame(self, bg=self.THEME['bg_main'], padx=20, pady=20)
        self.left_col.pack(side=tk.LEFT, fill=tk.Y, expand=False)
        
        self.right_col = tk.Frame(self, bg=self.THEME['bg_panel'], padx=25, pady=25)
        self.right_col.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)

        self.turn_label = tk.Label(self.left_col, text="", font=("Segoe UI", 18, "bold"), 
                                   bg=self.THEME['bg_main'], fg=self.THEME['fg_text'])
        self.turn_label.pack(pady=(0, 15))

        self.board_frame = tk.Frame(self.left_col, bg=self.THEME['board_bg'], bd=4, relief=tk.SUNKEN)
        self.board_frame.pack()
        
        self.board_buttons = [[None for _ in range(15)] for _ in range(15)]
        for r in range(15):
            for c in range(15):
                btn = tk.Button(
                    self.board_frame, width=3, height=1, font=("Segoe UI", 12, "bold"),
                    relief=tk.FLAT, bd=1,
                    command=lambda row=r, col=c: self.on_board_click(row, col)
                )
                btn.grid(row=r, column=c, padx=1, pady=1)
                self.board_buttons[r][c] = btn

        tk.Label(self.left_col, text="Active Rack (Click to place)", font=("Segoe UI", 12), 
                 bg=self.THEME['bg_main'], fg='#AAAAAA').pack(pady=(20, 5))
        
        self.rack_frame = tk.Frame(self.left_col, bg=self.THEME['bg_main'])
        self.rack_frame.pack()
        
        self.rack_buttons = []
        for i in range(7):
            btn = tk.Button(
                self.rack_frame, width=4, height=2, font=("Segoe UI", 14, "bold"),
                relief=tk.RAISED, bg=self.THEME['tile_placed'],
                command=lambda idx=i: self.on_rack_click(idx)
            )
            btn.grid(row=0, column=i, padx=4)
            self.rack_buttons.append(btn)

        self.control_frame = tk.Frame(self.left_col, bg=self.THEME['bg_main'])
        self.control_frame.pack(pady=20)
        
        tk.Button(self.control_frame, text="Submit Move", command=self.submit_move, 
                  font=("Segoe UI", 11, "bold"), bg=self.THEME['btn_submit'], fg='white', 
                  relief=tk.FLAT, padx=15, pady=5).pack(side=tk.LEFT, padx=10)
        
        tk.Button(self.control_frame, text="Clear Pending", command=self.clear_pending, 
                  font=("Segoe UI", 11, "bold"), bg=self.THEME['btn_clear'], fg='white', 
                  relief=tk.FLAT, padx=15, pady=5).pack(side=tk.LEFT, padx=10)

        tk.Label(self.right_col, text="Game State Inspector", font=("Segoe UI", 16, "bold"), 
                 bg=self.THEME['bg_panel'], fg=self.THEME['fg_text']).pack(anchor='w', pady=(0, 20))

        self.p1_lbl = tk.Label(self.right_col, text="", font=("Segoe UI", 12), 
                               bg=self.THEME['bg_panel'], fg=self.THEME['fg_text'], justify=tk.LEFT)
        self.p1_lbl.pack(anchor='w', pady=5)
        
        self.p2_lbl = tk.Label(self.right_col, text="", font=("Segoe UI", 12), 
                               bg=self.THEME['bg_panel'], fg=self.THEME['fg_text'], justify=tk.LEFT)
        self.p2_lbl.pack(anchor='w', pady=5)

        tk.Label(self.right_col, text="Tiles in Bag:", font=("Segoe UI", 12, "bold"), 
                 bg=self.THEME['bg_panel'], fg=self.THEME['fg_text']).pack(anchor='w', pady=(20, 5))
        
        self.bag_lbl = tk.Message(self.right_col, text="", font=("Consolas", 11), 
                                  bg='#2A2A2A', fg='#A5D6A7', width=300, 
                                  padx=10, pady=10, relief=tk.SUNKEN)
        self.bag_lbl.pack(anchor='w', fill=tk.X)

    def on_rack_click(self, index):
        display_rack = self._get_display_rack()
        if index < len(display_rack):
            self.selected_tile = display_rack[index]
            self.refresh()

    def on_board_click(self, r, c):
        if not self.selected_tile:
            return
        if self.placed_mask[r][c] or any(m[1] == (r, c) for m in self.pending_moves):
            return
            
        tile_to_place = self.selected_tile
        
        if tile_to_place == '[Blank]':
            letter = simpledialog.askstring("Blank Tile", "Enter letter for blank:")
            if not letter or len(letter) != 1 or not letter.isalpha():
                return
            tile_to_place = letter.lower() 

        self.pending_moves.append((tile_to_place, (r, c)))
        self.selected_tile = None
        self.refresh()

    def submit_move(self):
        if not self.pending_moves:
            return

        is_valid = self.game.play(self.current_player, self.pending_moves)
        
        if is_valid:
            for _, (r, c) in self.pending_moves:
                self.placed_mask[r][c] = True
            
            self.pending_moves.clear()
            self.selected_tile = None
            
            self.current_player, self.current_player_name = (
                (self.p2, "Player 2") if self.current_player == self.p1 else (self.p1, "Player 1")
            )
        else:
            messagebox.showwarning("Invalid Move", "Move rejected by game rules.")
            self.clear_pending()
            
        self.refresh()

    def clear_pending(self):
        self.pending_moves.clear()
        self.selected_tile = None
        self.refresh()

    def _get_display_rack(self):
        pending_counts = Counter(['[Blank]' if m[0].islower() else m[0] for m in self.pending_moves])
        display_rack = []
        for tile, count in self.current_player.rack.items():
            net_count = count - pending_counts.get(tile, 0)
            display_rack.extend([tile] * net_count)
        return display_rack

    def refresh(self):
        self.turn_label.config(text=f"Current Turn: {self.current_player_name}")

        self.p1_lbl.config(text=f"Player 1  |  Score: {self.p1.score}\nRack: {self.p1.rack_tiles}")
        self.p2_lbl.config(text=f"Player 2  |  Score: {self.p2.score}\nRack: {self.p2.rack_tiles}")
        
        bag_contents = [f"{k}:{v}" for k, v in self.game.bag.items() if v > 0]
        self.bag_lbl.config(text=", ".join(bag_contents) if bag_contents else "Bag is empty")

        pending_map = {coords: tile for tile, coords in self.pending_moves}

        for r in range(15):
            for c in range(15):
                cell_val = self.game.board[r][c]
                btn = self.board_buttons[r][c]
                
                if (r, c) in pending_map:
                    tile_char = pending_map[(r, c)]
                    fg_color = '#FF6347' if tile_char.islower() else 'black' 
                    btn.config(text=tile_char.upper(), bg=self.THEME['tile_pending'], fg=fg_color)
                elif self.placed_mask[r][c]:
                    fg_color = '#FF6347' if cell_val.islower() else 'black'
                    btn.config(text=cell_val.upper(), bg=self.THEME['tile_placed'], fg=fg_color)
                else:
                    mult_type = self.game.multipliers.get((r, c))
                    bg_color = self.BOARD_COLORS.get(mult_type, self.BOARD_COLORS['default'])
                    
                    if (r, c) == (7, 7):
                        display_text = '★'
                    else:
                        display_text = self.BOARD_LABELS.get(mult_type, '')
                        
                    btn.config(text=display_text, bg=bg_color, fg='#444444')

        display_rack = self._get_display_rack()
        for i, btn in enumerate(self.rack_buttons):
            if i < len(display_rack):
                btn.config(text=display_rack[i], state=tk.NORMAL)
                if self.selected_tile == display_rack[i] and not hasattr(self, '_selected_index_handled'):
                    btn.config(bg='#A5D6A7') 
                else:
                    btn.config(bg=self.THEME['tile_placed'])
            else:
                btn.config(text="", state=tk.DISABLED, bg="#555555")

if __name__ == "__main__":
    app = ScrabbleUI()
    app.mainloop()