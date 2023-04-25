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

    pub fn oopsie(&mut self) {
        self.snake.body = vec![SnakeCell(2028)]
    }

    // cannot return a reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //     self.snake.body
    // }

    pub fn update(&mut self) {

        let snake_idx = self.get_snake_head();
        let (row, col) = self.index_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Directions::Left => {
                (row, (col - 1) % self.width)
            },
            Directions::Right => {
                (row, (col + 1) % self.width)
            }, 
            Directions::Up => {
                ((row - 1) % self.width, col)
            },
            Directions::Down => {
                ((row + 1) % self.width, col)
            },  
        };
        self.snake.body[0].0 = self.cell_to_index(row, col);

        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head(next_idx);
    }

    

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}
