use core::fmt;
use std::{fs, thread, time::Duration};

const BATHROOM_WIDTH: i32 = 101;
const BATHROOM_HEIGHT: i32 = 103;

const INPUT_FILE_PATH: &str = "./input";
// const BATHROOM_WIDTH: i32 = 11;
// const BATHROOM_HEIGHT: i32 = 7;
const SIMULATION_DURATION: i32 = 1000;

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

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos: {}, Vel: {}", self.position, self.velocity)
    }
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
    if c >= threshold {
        c - threshold
    } else if c < 0 {
        c + threshold
    } else {
        c
    }
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
    robots
        .into_iter()
        .map(|robot| tick_robot_system(robot))
        .collect()
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

fn is_between_quadrants(point: &Vec2) -> bool {
    return false;
    point.x == BATHROOM_WIDTH / 2 || point.y == BATHROOM_HEIGHT / 2
}

fn print_robot_values(robots: &Vec<Robot>) {
    robots.into_iter().for_each(|robot| {
        println!("{}", robot);
    });
}

fn print_robots_grid(robots: &Vec<Robot>) {
    let mut output = String::new();
    for y in 0..BATHROOM_HEIGHT {
        for x in 0..BATHROOM_WIDTH {
            if is_between_quadrants(&Vec2 { x, y }) {
                output += " ";
                continue;
            }

            let robot_count = get_robot_count_in_range(Vec2 { x, y }, Vec2 { x, y }, &robots);

            if robot_count == 0 {
                output += "..";
            } else {
                output += "██";
            }
        }
        output += "\n";
    }
    print!("{}", output);
}

fn get_robot_count_in_range(min: Vec2, max: Vec2, robots: &Vec<Robot>) -> i32 {
    robots.into_iter().fold(0, |sum, robot| {
        let in_horizontal_range = min.x <= robot.position.x && robot.position.x <= max.x;
        let in_vertical_range = min.y <= robot.position.y && robot.position.y <= max.y;

        if is_between_quadrants(&robot.position) {
            sum
        } else if in_horizontal_range && in_vertical_range {
            sum + 1
        } else {
            sum
        }
    })
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_PATH).expect("Should have been able to read the file");
    println!("Parsed input:\n{contents}");

    let lines = contents.split("\n");

    let mut robots: Vec<Robot> = Vec::new();

    lines
        .into_iter()
        .for_each(|line| match extract_robot_from_line(line) {
            Some(robot) => robots.push(robot),
            None => {}
        });

    println!("Initial state:");
    print_robots_grid(&robots);

    for s in 0..SIMULATION_DURATION {
        robots = update_robots(&robots);
        println!("\nAfter {} seconds:", s + 1);
        if s > 100 {
            print_robots_grid(&robots);
        }
    }

    let q0 = get_robot_count_in_range(
        Vec2 { x: 0, y: 0 },
        Vec2 {
            x: BATHROOM_WIDTH / 2,
            y: BATHROOM_HEIGHT / 2,
        },
        &robots,
    );
    let q1 = get_robot_count_in_range(
        Vec2 {
            x: BATHROOM_WIDTH / 2,
            y: 0,
        },
        Vec2 {
            x: BATHROOM_WIDTH,
            y: BATHROOM_HEIGHT / 2,
        },
        &robots,
    );
    let q2 = get_robot_count_in_range(
        Vec2 {
            x: BATHROOM_WIDTH / 2,
            y: BATHROOM_HEIGHT / 2,
        },
        Vec2 {
            x: BATHROOM_WIDTH,
            y: BATHROOM_HEIGHT,
        },
        &robots,
    );
    let q3 = get_robot_count_in_range(
        Vec2 {
            x: 0,
            y: BATHROOM_HEIGHT / 2,
        },
        Vec2 {
            x: BATHROOM_WIDTH / 2,
            y: BATHROOM_HEIGHT,
        },
        &robots,
    );

    println!("");
    print_robot_values(&robots);
    print_robots_grid(&robots);

    println!(
        "\nTotal: {} from {}",
        get_robot_count_in_range(
            Vec2 { x: 0, y: 0 },
            Vec2 {
                x: BATHROOM_WIDTH,
                y: BATHROOM_HEIGHT
            },
            &robots
        ),
        robots.len()
    );

    let safety_number = q0 * q1 * q2 * q3;

    println!("{}, {}, {}, {}", q0, q1, q2, q3);

    println!("=> {}", safety_number);
}
