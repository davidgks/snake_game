use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Directions {
    Up, 
    Right,
    Down,
    Left,
}

#[wasm_bindgen]

struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Directions,
}

impl Snake{
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index)),
            direction: Directions::Up,
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
            snake: Snake::new(snake_idx)
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0    
    }



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
