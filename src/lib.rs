use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Directions {
    Up, 
    Right,
    Down,
    Left,
}

#[wasm_bindgen]

pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Directions,
}

impl Snake{
    fn new(spawn_index: usize, size: usize) -> Snake {

        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Directions::Right,
        }
    }
}


#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World { 
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3)
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0    
    }

    pub fn change_snake_direction(&mut self, direction: Directions) {
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // borrowing rules do not apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // cannot return a reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //     self.snake.body
    // }

    pub fn step(&mut self) {
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;
    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.get_snake_head();
        let row = snake_idx / self.width;
        let col = snake_idx - row * self.width;

        // return match self.snake.direction {
        //     Directions::Left => {
        //         SnakeCell((row * self.width) + (snake_idx - 1) % self.width)
        //     },
        //     Directions::Right => {
        //         SnakeCell((row * self.width) + (snake_idx + 1) % self.width)
        //     }, 
        //     Directions::Up => {
        //         SnakeCell((snake_idx - self.width) % self.size)
        //     },
        //     Directions::Down => {
        //         SnakeCell((snake_idx + self.width) % self.size)
        //     },  
        // };

        return match self.snake.direction {
            Directions::Left => {
                let threshold = row * self.width - 1;
                if snake_idx == threshold {
                    SnakeCell(threshold * (self.width - 1))
                } 
                else {
                    SnakeCell(snake_idx - 1)
                }   
            },
            Directions::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } 
                else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Directions::Up => {
                let threshold = col; 
                if snake_idx == threshold {
                    SnakeCell(self.size - (self.width - col))
                } 
                else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Directions::Down => {
                let threshold = self.size + col;
                if snake_idx == threshold {
                    SnakeCell(col)
                } 
                else {
                    SnakeCell(snake_idx + self.width)
                }
            },  
        } 
    }
}
