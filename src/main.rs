use std::{
    io::{self, Write, stdout}, thread, time::Duration
};

use crossterm::{
    cursor::MoveTo, event::{self, DisableFocusChange}, execute, terminal::{self, Clear, ClearType}
};
use crossterm::event::{poll, read, Event, KeyCode};
use rand::Rng;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    
    let mut snake = vec![(0, 0)];

    let mut food: Vec<(i32, i32)> = vec![];

    let mut direction = Dir::Right;

    loop {

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        
        if poll(Duration::from_millis(0))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('w') => direction = Dir::Up,
                    KeyCode::Char('a') => direction= Dir::Left,
                    KeyCode::Char('s') => direction= Dir::Down,
                    KeyCode::Char('d') => direction= Dir::Right,
                    // KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        if food.len() < 5 {
            food.push(spawn_food(&snake));
        }
        
        
        move_snake(&mut snake, &direction);
        
        if !update_game_and_has_collected(&mut snake, &mut food) {
            // letztes stück entfernt
            snake.remove(snake.len() - 1);
        }
        
        draw_game(&snake, &food);

        stdout().flush()?;

        thread::sleep(Duration::from_millis(100));
    }
}

fn update_game_and_has_collected(snake: &mut Vec<(i32, i32)>, food: &mut Vec<(i32, i32)>) -> bool {
    for pos in snake {
        if food.contains(pos) {
            if let Some(index) = food.iter().position(|p| p == pos) {
                food.remove(index);
            }
            return true;
        }
    }
    false
}

fn draw_game(snake: &Vec<(i32, i32)>, food: &Vec<(i32, i32)>) {
    for y in 0..20 {
        for x in 0..30 {
            if snake.contains(&(x, y)) {
                if snake[0] == ((x, y)) {
                    print!("%");
                } else {
                    print!("@");
                }
            } else if food.contains(&(x, y)) {
                print!("*")
            } else {
                print!(".");
            }
        }
        print!("\r\n")
    }
}
fn check_game_over(snake: &Vec<(i32, i32)>) -> bool {
    if snake[0].0 > 30 || snake[0].1 > 20 // Rechts, unten
    || snake[0].0 < 0 
     
    {
        return true;
    }
    false
}

fn move_snake(snake: &mut Vec<(i32, i32)>, dir: &Dir) {
    let last_snake_pos = snake[0];
    match dir {
        Dir::Down => if last_snake_pos.1 + 1 <= 20 { 
            snake.insert(0,(last_snake_pos.0, last_snake_pos.1 + 1))
        },
        Dir::Left => if last_snake_pos.0 - 1 >= 0 { 
                snake.insert(0, (last_snake_pos.0 - 1, last_snake_pos.1))
        },
        Dir::Right => if last_snake_pos.0 + 1 < 30 { 
                snake.insert(0, (last_snake_pos.0 + 1, last_snake_pos.1))
        },
        Dir::Up => if last_snake_pos.1 - 1 >= 0 { 
                snake.insert(0, (last_snake_pos.0, last_snake_pos.1 - 1))
        },
    }
}

fn contains_snake(snake: &Vec<(i32, i32)>, pos: &(i32, i32)) -> bool {
    snake.contains(pos)
}

fn spawn_food(snake: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut rng = rand::rng();

    loop {
        let pos = (
            rng.random_range(0..20),
            rng.random_range(0..30),
        );

        if !snake.contains(&pos) {
            return pos;
        }
    }
}