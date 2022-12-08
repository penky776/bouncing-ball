use std::io::Write;
use std::{thread, time};

struct Coordinates(u16, u16);

impl Coordinates {
    fn add_one_to_y(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn subtract_one_from_y(&self) -> Self {
        Self(self.0, self.1 - 1)
    }
}

fn main() {
    let ground = Coordinates(53, 5);
    let top = Coordinates(53, 1);
    let mut current_position = Coordinates(53, 0);

    print!(
        "{}Starting...{}",
        termion::clear::All,
        termion::cursor::Hide
    );
    std::io::stdout().flush().unwrap();
    thread::sleep(time::Duration::from_secs(2));

    // incrementing and reducing the value of only y - as of now, it changes by 1 unit per iteration.
    loop {
        while current_position.1 < ground.1 {
            current_position = Coordinates::add_one_to_y(&current_position);

            print!(
                "{}{}O",
                termion::clear::All,
                termion::cursor::Goto(current_position.0, current_position.1)
            );

            std::io::stdout().flush().unwrap();

            print!(
                " coordinates: ({}, {})",
                current_position.0, current_position.1
            );
            std::io::stdout().flush().unwrap();

            thread::sleep(time::Duration::from_millis(500));
        }

        while current_position.1 > top.1 {
            current_position = Coordinates::subtract_one_from_y(&current_position);

            print!(
                "{}{}O",
                termion::clear::All,
                termion::cursor::Goto(current_position.0, current_position.1)
            );

            std::io::stdout().flush().unwrap();

            print!(
                " coordinates: ({}, {})",
                current_position.0, current_position.1
            );
            std::io::stdout().flush().unwrap();

            thread::sleep(time::Duration::from_millis(500));
        }
    }
}
