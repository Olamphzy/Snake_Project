use std::collections::LinkedList;   
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;  //bring in draw_block from draw.rs file

//Snake Color
const SNAKE_COLOR: Color = [0.00, 1.00, 0.00, 1.0];  //Green snake with 1.0 opaity

//Handle the direction of snake and how keyboard interact with snake
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Implement method for enum Is method that match the description that if the snake goes up and down is hit down arrow, it won't go down
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]
struct Block {  // doesn't need to be public
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,  //tail will be actual value when it eat an apple
}

// Implementation block for snake
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y,
        });  // when will start game snake will be length of 3 with horizntal, moving to the right
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });
    // Snake will be horinzontal with x and y coordinates
        Snake {
            direction: Direction::Right,  // Starting moving in direction of right and tail will be None
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body { //iterate through list
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);  //This will render out a green snake
        }
    }

    pub fn head_position(&self) -> (i32, i32) { //tuple of i32
        let head_block = self.body.front().unwrap(); 
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1, //move forward in negative y axes
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();  //unwrap so we don't have an error
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk); //if we eat apple this method will be called with snake length increase
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {  // if snake is overlap any part of its body
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 { // check snake length where the tail and head exist
                break;
            }
        }
        return false;
    }
}
