use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]

struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>
}

impl Snake{
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index))
        }
    }
}


#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        let width = 8;
        World { 
            width,
            size: width * width,
            snake: Snake::new(10)
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
        self.snake.body[0].0 = (snake_idx + 1) % self.size;

        // squares = {0, ..., 63} -> world.width * world.widthm - 1
        // 66 % 64 = 2
    

        

        
    }
}
