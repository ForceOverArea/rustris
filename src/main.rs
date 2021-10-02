use rustris::{ Tetramino, Board };

fn main(){
    let my_piece = Tetramino { 
        shape: 'T', 
        angle: 0, 
    };

    let my_piece2 = Tetramino { 
        shape: 'Z', 
        angle: 0, 
    };

    let my_board = Board::new();

    // println!("{}", my_board.display(my_piece2, 5, 10));
    // println!("{}", my_board.display(my_piece, 5, 10));
    my_board.commit(my_piece, 5, 10);
    println!("{}", my_board.display(my_piece2, 5, 15));
}

mod rustris {
    #[derive(Copy, Clone)]
    pub struct Tetramino {
        pub shape: char,
        pub angle: u8,
    }
    impl Tetramino {
        #[allow(dead_code)]
        fn rotate(mut self, clockwise: bool) {
            if clockwise {
                if self.angle < 3 {
                    self.angle += 1;
                } else {
                    self.angle = 0; // In case the angle is already 3
                }
            } else {
                if self.angle > 0 {
                    self.angle -= 1;
                } else {
                    self.angle = 3; // In case the angle is already 0
                }
                self.angle -= 1;
            }
        }
        #[allow(dead_code)]
        fn render(self) -> [[char; 4]; 4] {
            let mut sprite = [['.'; 4]; 4];
            if self.shape == 'T' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['.','T','T','T'],
                        ['.','.','T','.']
                    ];
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','T','.'],
                        ['.','.','T','T'],
                        ['.','.','T','.']
                    ];
                } else if self.angle == 2 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','T','.'],
                        ['.','T','T','T'],
                        ['.','.','.','.']
                    ];
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','T','.'],
                        ['.','T','T','.'],
                        ['.','.','T','.']
                    ];
                }
            } else if self.shape == 'O' {
                sprite = [
                    ['.','.','.','.'],
                    ['.','O','O','.'],
                    ['.','O','O','.'],
                    ['.','.','.','.'],
                ]
            } else if self.shape == 'I' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','I','.'],
                        ['.','.','I','.'],
                        ['.','.','I','.'],
                        ['.','.','I','.']
                    ]
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['I','I','I','I'],
                        ['.','.','.','.'],
                        ['.','.','.','.']
                    ]
                } else if self.angle == 2 {
                    sprite = [
                        ['.','I','.','.'],
                        ['.','I','.','.'],
                        ['.','I','.','.'],
                        ['.','I','.','.']
                    ]
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['I','I','I','I'],
                        ['.','.','.','.']
                    ]
                }
            } else if self.shape == 'J' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','J','.'],
                        ['.','.','J','.'],
                        ['.','J','J','.']
                    ]
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','J','.','.'],
                        ['.','J','J','J'],
                        ['.','.','.','.']
                    ]
                } else if self.angle == 2 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','J','J'],
                        ['.','.','J','.'],
                        ['.','.','J','.']
                    ]
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['.','J','J','J'],
                        ['.','.','.','J']
                    ]
                }
            } else if self.shape == 'L' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','L','.'],
                        ['.','.','L','.'],
                        ['.','.','L','L']
                    ]
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['.','L','L','L'],
                        ['.','L','.','.']
                    ]
                } else if self.angle == 2 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','L','L','.'],
                        ['.','.','L','.'],
                        ['.','.','L','.']
                    ]
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','L'],
                        ['.','L','L','L'],
                        ['.','.','.','.']
                    ]
                }
            } else if self.shape == 'S' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['.','.','S','S'],
                        ['.','S','S','.']
                    ]
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','S','.'],
                        ['.','.','S','S'],
                        ['.','.','.','S']
                    ]
                } else if self.angle == 2 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','S','S'],
                        ['.','S','S','.'],
                        ['.','.','.','.']
                    ]
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','S','.','.'],
                        ['.','S','S','.'],
                        ['.','.','S','.']
                    ]
                }
            } else if self.shape == 'Z' {
                if self.angle == 0 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','.'],
                        ['.','Z','Z','.'],
                        ['.','.','Z','Z']
                    ]
                } else if self.angle == 1 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','.','Z'],
                        ['.','.','Z','Z'],
                        ['.','.','Z','.']
                    ]
                } else if self.angle == 2 {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','Z','Z','.'],
                        ['.','.','Z','Z'],
                        ['.','.','.','.']
                    ]
                } else {
                    sprite = [
                        ['.','.','.','.'],
                        ['.','.','Z','.'],
                        ['.','Z','Z','.'],
                        ['.','Z','.','.']
                    ]
                }
            }
            sprite
        }
    }

    #[derive(Copy, Clone)]
    pub struct Board {
        pub state: [[char; 10]; 20],
    }
    impl Board {
        pub fn new() -> Board {
            Board { 
                state: [['.'; 10]; 20],
            }
        }

        pub fn display(self, piece: Tetramino, curs_x:usize, curs_y:usize) -> String { 
            
            let mut graphical_out = "".to_string();
            let mut temp_graphics = self.state; //prevents overwriting the state attribute

            for j in 0..20 {
                for i in 0..10 {
                    if j >= curs_y  && j < curs_y + 4 && i >= curs_x && i < curs_x + 4 { 

                        let sprite_j = j - curs_y;
                        let sprite_i = i - curs_x;
                        let sprite = piece.render();
                        
                        if sprite[sprite_j][sprite_i] != '.' && self.state[j][i] != '.' { 
                            return "Error".to_string();
                        } else {
                            temp_graphics[j][i] = sprite[sprite_j][sprite_i];
                        }  
                        graphical_out.push( sprite[sprite_j][sprite_i] );
                    } else { 
                        graphical_out.push( temp_graphics[j][i] )
                    }
                }
                graphical_out.push('\n')
            }
        graphical_out
        }

        #[allow(dead_code)]
        pub fn commit(mut self, piece: Tetramino, curs_x:usize, curs_y:usize) -> () { // commit the current graphics to the board.state property. This method returns ()

            let status = self.display(piece, curs_x, curs_y); // returns valid graphics or string "Error"
            
            if status != "Error" {
                println!("Committing tetramino to state...");
                for j in 0..20 {
                    for i in 0..10 {
                        if j >= curs_y  && j < curs_y + 4 && i >= curs_x && i < curs_x + 4 { // if x and y are in the area of a sprite, cheak if it can be committed, then commit
    
                            let sprite_j = j - curs_y;
                            let sprite_i = i - curs_x;
                            let sprite = piece.render();
                            
                            self.state[j][i] = sprite[sprite_j][sprite_i];
                            println!("{}",self.state[j][i]);
                        } 
                    }
                }
                println!("Done!")
            } else {
                println!("ERR: Failed to commit to state!");
                return ()
            }
        }
    }
}