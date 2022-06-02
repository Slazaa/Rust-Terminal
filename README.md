# Rust - Terminal
A Rust cross-platform terminal manager

## Implemented OS
- Windows

## Features
- Cross-platform
- Terminal
	- Clear
	- Size
	- Title
- Cursor
	- Position
	- Visibility
- Style
	- Foreground color
	- Background color
- Event
	- Read event

## Examples
Terminal:
```rs
use terminal::terminal;

fn main() {
	terminal::clear();
	terminal::get_size();
	terminal::get_title();
	terminal::set_size(Size::new(50, 50));
	terminal::set_title("New Title");
}
```

Cursor:
```rs
use terminal::cursor;

fn main() {
	cursor::get_pos();
	cursor::is_visible();
	cursor::set_pos(Position::new(10, 10));
	cursor::set_visible(false);
}
```

Style:
```rs
use terminal::style{self, Color};

fn main() {
	style::get_background_color();
	style::get_foreground_color();
	style::set_background_color(Color::Red);
	style::set_foreground_color(Color::Blue);
}
```

Event:
```rs
use terminal::event::{self, Event};

fn main() {
    loop {
        match event::read().unwrap() {
            Event::Key { key, down } => {
                if down {
                    println!("{} is pressed", key);

                    if key == 'a' {
                        break;
                    }
                }
            }
            _ => ()
        }
    }
}
```

## Libraries Used
* [winapi](https://github.com/retep998/winapi-rs)
