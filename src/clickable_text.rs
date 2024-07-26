use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, EnableMouseCapture, DisableMouseCapture, MouseEventKind, MouseButton},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::stdout;

use std::error::Error;

pub fn print_clickable(text1: &str, text2: &str) -> Result<i8, Box<dyn Error>> {
    println!("");
    println!("[{text1}] [{text2}]");
    let (_, r) = crossterm::cursor::position()?;
    let r = r as usize;
    enable_raw_mode().unwrap();
    stdout().execute(EnableMouseCapture);
    let ans = handle_button(r - 1, 1, 1 + text1.len(), 4 + text1.len(), 4 + text1.len() + text2.len()); 
    stdout().execute(DisableMouseCapture);
    disable_raw_mode().unwrap();
    println!("");
    ans
}

fn handle_button(r: usize, l1: usize, r1: usize, l2: usize, r2: usize) -> Result<i8, Box<dyn Error>> {
    loop {
        let event = read()?;
        if let Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            modifiers,
            column,
            row,
        }) = event
        {
            if row as usize == r && column as usize >= l1 && column as usize <= r1 {
                return Ok(1)
            }
            if row as usize == r && column as usize >= l2 && column as usize <= r2 {
                return Ok(2)
            }
        }
        
        if let Event::Key(KeyEvent { code, modifiers, kind, state}) = event {
            if code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                break;
            }
        }
    }
    Ok(0)
}
