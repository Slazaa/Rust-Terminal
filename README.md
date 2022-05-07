# Rust - Terminal Manager
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

## Examples
Terminal

```rs
use terminal_manager::terminal;

fn main() {
	terminal::clear();
	terminal::get_size();
	terminal::get_title();
	terminal::set_size(Size::new(50, 50));
	terminal::set_title("New Title");
}
```

## Libraries Used
* [winapi](https://github.com/retep998/winapi-rs)