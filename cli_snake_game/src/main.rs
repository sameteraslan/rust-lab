use crossterm::{
    cursor, event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use log::{info, LevelFilter};
use simplelog::*;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use rand::Rng;

// Game structure representing the snake game
struct Game {
    snake: Vec<(u16, u16)>, // Snake's positions (x, y coordinates)
    direction: Direction,   // Snake's movement direction
    food: (u16, u16),       // Food position
    width: u16,             // Game area width
    height: u16,            // Game area height
    score: u16,             // Game score
    score_width: u16,       // Scoreboard width
}

// Enumeration to define movement directions
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    // Initialize a new game instance
    fn new(width: u16, height: u16, score_w: u16) -> Self {
        Self {
            snake: vec![(width / 2, height / 2)], // Start position at the center
            direction: Direction::Right,          // Initial direction
            food: (width / 4, height / 4),        // Initial food position
            width,
            height,
            score: 0,
            score_width: score_w,
        }
    }

    // Update the game state: move the snake, check collisions, and handle scoring
    fn update(&mut self) -> Result<(), &'static str> {
        let mut new_head = *self.snake.first().unwrap();

        // Calculate new head position based on movement direction
        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }

        // Check if the snake eats the food
        if new_head == self.food {
            self.score += 1;
            self.food = (
                rand::thread_rng().gen_range(1..self.width),  // Generate new food position
                rand::thread_rng().gen_range(1..self.height),
            );
        } else {
            self.snake.pop(); // Remove the tail if food isn't eaten
        }

        // Add new head position
        self.snake.insert(0, new_head);
        info!("new_head.0: {}, new_head.1: {}", new_head.0, new_head.1);

        // Check for boundary collision
        if new_head.0 == 0 || new_head.0 >= self.width || new_head.1 == 0 || new_head.1 >= self.height {
            return Err("Game Over");
        }
        Ok(())
    }

    // Change the snake's direction
    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    // Draw game borders on the terminal
    fn draw_borders(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        for x in 0..=self.width {
            execute!(stdout, cursor::MoveTo(x, 0))?; // Top border
            print!("#");
            execute!(stdout, cursor::MoveTo(x, self.height))?; // Bottom border
            print!("#");
        }
        for y in 0..=self.height {
            execute!(stdout, cursor::MoveTo(0, y))?; // Left border
            print!("#");
            execute!(stdout, cursor::MoveTo(self.width, y))?; // Right border
            print!("#");
        }

        Ok(())
    }

    // Draw the score and scoreboard on the terminal
    fn draw_score(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        for x in self.width..=self.width + self.score_width {
            execute!(stdout, cursor::MoveTo(x, 0))?; // Top border
            print!("#");
            execute!(stdout, cursor::MoveTo(x, self.height))?; // Bottom border
            print!("#");
        }
        for y in 0..=self.height {
            execute!(stdout, cursor::MoveTo(self.width + self.score_width, y))?; // Right border
            print!("#");
        }

        execute!(stdout, cursor::MoveTo(self.width + self.score_width / 2 - 2, self.height / 2 - 1))?;
        print!("Score");
        execute!(stdout, cursor::MoveTo(self.width + self.score_width / 2 - self.score.to_string().chars().count() as u16 / 2, self.height / 2))?;
        print!("{}", self.score);

        Ok(())
    }

    // Draw the game state including borders, snake, and food
    fn draw(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?; // Clear the screen

        self.draw_borders()?;
        self.draw_score()?;

        for &(x, y) in &self.snake {
            execute!(stdout, cursor::MoveTo(x, y))?;
            print!("O"); // Draw the snake
        }

        execute!(stdout, cursor::MoveTo(self.food.0, self.food.1))?;
        print!("*"); // Draw the food

        stdout.flush()?;
        Ok(())
    }

    // Display the end game screen with the final score
    fn end_game(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let game_over = "Game Over!";
        let thanks = "Thanks for playing!";
        let score = "Your score is ";

        execute!(stdout, terminal::Clear(ClearType::All))?;
        self.draw_borders()?;
        execute!(stdout, cursor::MoveTo((self.width - game_over.len() as u16) / 2, self.height / 2 - 1))?;
        println!("{}", game_over);
        execute!(stdout, cursor::MoveTo((self.width - thanks.len() as u16) / 2, self.height / 2))?;
        println!("{}", thanks);
        execute!(stdout, cursor::MoveTo((self.width - score.len() as u16 - self.score.to_string().chars().count() as u16) / 2, self.height / 2 + 1))?;
        println!("{}{}", score, self.score);
        std::thread::sleep(Duration::from_secs(3));

        Ok(())
    }
}

// Main function to initialize and run the game
fn main() -> Result<(), std::io::Error> {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("app.log")
                .unwrap(),
        ),
    ])
    .unwrap();

    let file_path = "./config/configuration.json";
    let mut file = File::open(file_path).expect("Unable to open the JSON file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the JSON file");

    let json_data: Value = serde_json::from_str(&contents).expect("Unable to parse JSON");
    let game_speed = Duration::from_millis(json_data.get("game_speed").unwrap().as_u64().unwrap());
    let dynamic_game_speed = json_data.get("dynamic_game_speed").unwrap().as_bool().unwrap();
    let screen_width = json_data.get("screen_width").unwrap().as_u64().unwrap();
    let screen_height = json_data.get("screen_height").unwrap().as_u64().unwrap();
    let score_width = json_data.get("score_width").unwrap().as_u64().unwrap();

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    let welcome = "Welcome to Snake Game!";
    let start_playing = "Press 'P' to Play";

    let mut game = Game::new(screen_width.try_into().unwrap(), screen_height.try_into().unwrap(), score_width.try_into().unwrap());

    execute!(stdout, terminal::Clear(ClearType::All))?;
    game.draw_borders()?;
    execute!(stdout, cursor::MoveTo((game.width - welcome.len() as u16) / 2, game.height / 2))?;
    println!("{}", welcome);
    execute!(stdout, cursor::MoveTo((game.width - start_playing.len() as u16) / 2, game.height / 2 + 1))?;
    println!("{}", start_playing);

    loop {
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                if matches!(key_event.code, KeyCode::Char('p' | 'P')) {
                    break;
                }
            }
        }
    }

    let mut prev_key_code = KeyCode::Null;
    let mut last_update = Instant::now();
    let mut game_speed_time = Duration::from_millis(1000);

    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if prev_key_code != key_event.code && !matches!((prev_key_code, key_event.code),
                    (KeyCode::Left, KeyCode::Right) | (KeyCode::Right, KeyCode::Left) |
                    (KeyCode::Up, KeyCode::Down) | (KeyCode::Down, KeyCode::Up)) {
                    info!("prev_key: {:?}, key_event: {:?}", prev_key_code, key_event.code);
                    prev_key_code = key_event.code;
                    match key_event.code {
                        KeyCode::Esc => {
                            game.end_game()?;
                            break;
                        }
                        KeyCode::Up => game.change_direction(Direction::Up),
                        KeyCode::Down => game.change_direction(Direction::Down),
                        KeyCode::Left => game.change_direction(Direction::Left),
                        KeyCode::Right => game.change_direction(Direction::Right),
                        _ => {}
                    }
                }
            }
        }

        if last_update.elapsed() >= game_speed_time {
            if let Err(err) = game.update() {
                info!("Error: {}", err);
                game.end_game()?;
                break;
            }
            game.draw()?;
            last_update = Instant::now();
        }

        if dynamic_game_speed {
            if game_speed_time > Duration::from_millis((200 - game.score).into()) {
                game_speed_time = game_speed_time - game_speed;
            } 
        }
    }

    execute!(stdout, cursor::MoveTo(0, game.height + 1))?;
    terminal::disable_raw_mode()?;
    Ok(())
}
