use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module="/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Directions {
    Up, 
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Clone, Copy)]
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
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {

        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        World { 
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
          reward_cell = rnd(max);
          if !snake_body.contains(&SnakeCell(reward_cell)) { break; }
        }

        reward_cell
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0    
    }

    pub fn change_snake_direction(&mut self, direction: Directions) {

        let next_cell = self.gen_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&mut self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // borrowing rules do not apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let temp = self.snake.body.clone(); // copies the vector

        match self.next_cell {
            Some(cell) => { 
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        let len = self.snake_length();
        // update all other cells of snake body
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i-1].0);
        }
        
        if self.snake.body[0].0 == self.reward_cell {
            self.snake.body.push(SnakeCell(self.snake.body[1].0));
            self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
        }
    }

    fn gen_next_snake_cell(&self, direction: &Directions) -> SnakeCell {
        let snake_idx = self.get_snake_head();
        let row = snake_idx / self.width;
        let col = snake_idx - row * self.width;

        return match direction {
            Directions::Left => {
                let threshold = row * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.width - 1))
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
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },  
        }
    }
}
