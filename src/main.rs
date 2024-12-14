use std::env;
use std::fs;

const BATHROOM_WIDTH: i32 = 101;
const BATHROOM_HEIGHT: i32 = 103;

#[derive(Debug, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

fn extract_value(string: &str) -> Vec2 {
    let split: Vec<&str> = string.split(",").collect();

    let x_part = split.get(0).unwrap();
    let y_part = split.get(1).unwrap();

    let x_string_parts: Vec<&str> = x_part.split("=").collect();
    let x_part = x_string_parts.get(1).unwrap();

    let x: i32 = x_part.parse().unwrap();
    let y: i32 = y_part.parse().unwrap();

    Vec2 { x, y }
}

fn wrap_value(a: i32, b: i32, threshold: i32) -> i32 {
    let c = a + b;
    if c > threshold {
        return 0;
    }

    c
}

fn tick_robot_system(robot: &Robot) -> Robot {
    Robot {
        position: Vec2 {
            x: wrap_value(robot.position.x, robot.velocity.x, BATHROOM_WIDTH),
            y: wrap_value(robot.position.y, robot.velocity.y, BATHROOM_HEIGHT),
        },
        velocity: Vec2 {
            x: robot.velocity.x,
            y: robot.velocity.y,
        },
    }
}

fn update_robots(robots: &Vec<Robot>) -> Vec<Robot> {
    robots.into_iter().map(|robot| tick_robot_system(robot)).collect()
}

fn extract_robot_from_line(line: &str) -> Option<Robot> {
    if line == "" {
        return None;
    }

    let split: Vec<&str> = line.split(" ").collect();
    let pos = extract_value(split.get(0).unwrap());
    let vel = extract_value(split.get(1).unwrap());

    let robot = Robot {
        position: pos,
        velocity: vel,
    };

    Some(robot)
}

fn print_robots(robots: &Vec<Robot>) {
    for y in 0..BATHROOM_HEIGHT {
        for x in 0..BATHROOM_WIDTH {
            let robot_count = robots.clone().into_iter().fold(0, |sum, robot| {
                if robot.position.x == x && robot.position.y == y {
                    sum + 1
                } else {
                    sum
                }
            });

            if robot_count == 0 {
                print!(".");
            } else {
                print!("{}", robot_count);
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");
    println!("Parsed input:\n{contents}");

    let lines = contents.split("\n");

    let mut robots: Vec<Robot> = Vec::new();

    lines.into_iter().for_each(|line| {
        match extract_robot_from_line(line) {
            Some(robot) => robots.push(robot),
            None => {}
        }
    });

    print_robots(&robots);

    let mut new_robots = robots;

    for i in 0..100 {
        new_robots = update_robots(&new_robots);
    }

    print_robots(&new_robots);
}
