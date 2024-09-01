use std::{collections::HashMap, thread, time::Duration, usize};
use ::rand::random;

use macroquad::prelude::*;

const SIZE_X: usize = 72;
const SIZE_Y: usize = 40;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Snake {
    snake: Vec<(usize, usize)>
}

impl Snake {
    pub fn new() -> Snake {
        return Snake{ snake: vec![(0,0)] };
    }

    fn turn(&mut self, d: Direction) {
        //println!("Thinks to go: {:#?}", d);
        let c: (usize, usize) = self.snake[0];
        match d {
            Direction::UP => {
                let mut n: Vec<(usize, usize)> = vec![(c.0, c.1 + 1)];
                n.append(&mut self.snake);
                self.snake = n;
            },
            Direction::DOWN => {
                let mut n: Vec<(usize, usize)> = vec![(c.0, c.1 - 1)];
                n.append(&mut self.snake);
                self.snake = n;
            },
            Direction::LEFT => {
                let mut n: Vec<(usize, usize)> = vec![(c.0 - 1, c.1)];
                n.append(&mut self.snake);
                self.snake = n;
            },
            Direction::RIGHT => {
                let mut n: Vec<(usize, usize)> = vec![(c.0 + 1, c.1)];
                n.append(&mut self.snake);
                self.snake = n;
            },
        }
        self.snake.pop();
    }

    fn deside(&self, options: Vec<(usize, usize)>, h: &HashMap<(usize, usize), usize>, apple: (usize, usize), size: usize) -> Direction {
        //println!("Thinks position is totally: {:?}", self.snake[0]);
        let c: usize = *h.get(&self.snake[0]).unwrap();
        let a: usize = *h.get(&apple).unwrap();
        let n: usize;
        if size == c {
            n = 0;
        } else {
            n = c + 1;
        }

        for i in &options {
            let l: usize = *h.get(&i).unwrap_or(&usize::MAX);
            if l != n && (l <= a || (*h.get(&self.snake[self.snake.len() - 1]).unwrap_or(&0) > a && *h.get(&self.snake[self.snake.len() - 1]).unwrap_or(&0) < c)) && l > c && l - c > (self.snake.len() as f64 * 0.4) as usize {
                let head: (usize, usize) = self.snake[0];
                if head.0 != i.0 {
                    // this makes sense if you think about it, this is the comment that will tell you that this is mildly annoying to figure out
                    if head.0 > i.0 {
                        return Direction::LEFT;
                    } else {
                        return Direction::RIGHT;
                    }
                } else {
                    if head.1 > i.1 {
                        return Direction::DOWN;
                    } else {
                        return Direction::UP;
                    }
                }
            }
        }

        /* The else statment */
        //println!("Wants to go to: {:?}", n);
        for i in options {
            if *h.get(&i).unwrap_or(&usize::MAX) == n {
                //println!("Sees {:?} at: {:?}", n, i);
                let head: (usize, usize) = self.snake[0];
                if head.0 != i.0 {
                    // this makes sense if you think about it, this is the comment that will tell you that this is mildly annoying to figure out
                    if head.0 > i.0 {
                        return Direction::LEFT;
                    } else {
                        return Direction::RIGHT;
                    }
                } else {
                    if head.1 > i.1 {
                        return Direction::DOWN;
                    } else {
                        return Direction::UP;
                    }
                }
            }
        }
        /* `Direction` value */
        return Direction::UP;
    }

    fn extend(&mut self) {
        self.snake.push(self.snake[self.snake.len() - 1]);
    }
}

fn hamiltonian(width: usize, height: usize, h: &mut HashMap<(usize,usize), usize>) {
    if width % 2 == 0 {
        // normall
        let mut wx: usize = 0;
        let mut hy: usize = 0;
        let mut counter: usize = 0;
        let mut going_down: bool = true;
        let mut going_left: bool = false;

        loop {
            if h.contains_key(&(wx,hy)) {
                println!("Stop at: {:?}", (wx, hy));
                break;
            }
            h.insert((wx, hy), counter);
            //println!("Set {:?} to {:?}", (wx, hy), counter);
            counter += 1;
            if !going_left {
                if going_down {
                    hy += 1;
                    if hy >= height {
                        hy = height - 1;
                        wx += 1;
                        going_down = false;
                    }
                } else {
                    hy -= 1;
                    if hy < 1 {
                        hy = 1;
                        wx += 1;
                        going_down = true;
                        if wx >= width {
                            going_left = true;
                            hy -= 1;
                            wx -= 1;
                        }
                    }
                }
            } else {
                wx -= 1;
            }
        }

    } else {
        // add squigel at the end
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut s: Snake = Snake::new();

    let mut h: HashMap<(usize, usize), usize> = HashMap::new();

    let mut apple: (usize, usize) = (5, 5);

    hamiltonian(SIZE_X, SIZE_Y, &mut h);

    loop {
        clear_background(BLACK);

        let rect_size: f32 = (screen_width() / SIZE_X as f32) - 5.0;

        for x in 0..SIZE_X {
            for y in 0..SIZE_Y {
                if s.snake[0] == (x,y) {
                    draw_rectangle((rect_size + 5.0) * x as f32, (rect_size + 5.0) * y as f32, rect_size, rect_size, BLUE);
                } else if s.snake.contains(&(x, y)) {
                    draw_rectangle((rect_size + 5.0) * x as f32, (rect_size + 5.0) * y as f32, rect_size, rect_size, GREEN);
                    //draw_text(format!("{:?}", h.get(&(x,y)).unwrap_or(&usize::MAX)).as_str(), ((rect_size + 5.0) * x as f32) + (rect_size / 2.0), ((rect_size + 5.0) * y as f32) + (rect_size / 2.0), 25.0, WHITE);
                } else if apple == (x,y) {
                    draw_rectangle((rect_size + 5.0) * x as f32, (rect_size + 5.0) * y as f32, rect_size, rect_size, RED);
                } else {
                    draw_rectangle((rect_size + 5.0) * x as f32, (rect_size + 5.0) * y as f32, rect_size, rect_size, GRAY);
                    //draw_text(format!("{:?}", h.get(&(x,y)).unwrap_or(&usize::MAX)).as_str(), ((rect_size + 5.0) * x as f32) + (rect_size / 2.0), ((rect_size + 5.0) * y as f32) + (rect_size / 2.0), 25.0, WHITE);
                }
            }
            for x in 1..s.snake.len() {
                if s.snake[0] == s.snake[x] {
                    draw_rectangle((rect_size + 5.0) * s.snake[x].0 as f32, (rect_size + 5.0) * s.snake[x].1 as f32, rect_size, rect_size, YELLOW);
                    //println!("BAD!");
                }
            }
        }

        let mut options: Vec<(usize, usize)> = Vec::new(); 
        if h.contains_key(&(s.snake[0].0 + 1, s.snake[0].1)) && !s.snake.contains(&(s.snake[0].0 + 1, s.snake[0].1)) {
            options.push((s.snake[0].0 + 1, s.snake[0].1));
        }
        if h.contains_key(&(s.snake[0].0, s.snake[0].1 + 1)) && !s.snake.contains(&(s.snake[0].0, s.snake[0].1 + 1)) {
            options.push((s.snake[0].0, s.snake[0].1 + 1));
        }
        if s.snake[0].0 > 0 && h.contains_key(&(s.snake[0].0 - 1, s.snake[0].1)) && !s.snake.contains(&(s.snake[0].0 - 1, s.snake[0].1)) {
            options.push((s.snake[0].0 - 1, s.snake[0].1));
        }
        if s.snake[0].1 > 0 && h.contains_key(&(s.snake[0].0, s.snake[0].1 - 1)) && !s.snake.contains(&(s.snake[0].0, s.snake[0].1 - 1)) {
            options.push((s.snake[0].0, s.snake[0].1 - 1));
        }

        if s.snake[0] == apple {
            loop {
                let x: usize = (random::<f64>() * SIZE_X as f64) as usize;
                let y: usize = (random::<f64>() * SIZE_Y as f64) as usize;
                apple = (x,y);
                if !s.snake.contains(&apple) || s.snake.len() >= SIZE_X * SIZE_Y {
                    break;
                }
            }
            //println!("Apple is now at: {:?}", apple);
            s.extend();
        }

        let n = s.deside(options, &h, apple, (SIZE_X * SIZE_Y) - 1);

        s.turn(n);

        // for slow humans to see whats happening
        //thread::sleep(Duration::from_millis(20));

        if s.snake.len() >= SIZE_X * SIZE_Y {
            break;
        }

        next_frame().await
    }
}
