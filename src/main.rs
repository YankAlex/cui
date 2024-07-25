use cui::*;
use cui::elements::{Border, Container};
use std::sync::Arc;
use elements::Button;

fn main() {
    let mut app = App::new(|command, app| {
        match command.as_str() {
            _ => {}
        }
    });
    let mut block = elements::Block::new(30, 30, Border::Bold, 1, "name of block", "block1", 2, 2);
    block.push_element(Box::new(Button::new("i\n".to_string(), Arc::new(|app| {
        *app.lock().unwrap().get_element_by_id("block1").unwrap().into_container().unwrap().get_element_by_id("button2").unwrap().r() += 1;
        app.lock().unwrap().redraw();
    }), "Button in block", Border::Slim, 0, "button1".to_string(), 2, 2)));
    block.push_element(Box::new(Button::new("j\n".to_string(), Arc::new(|app| {}), "Button in block", Border::Slim, 0, "button2".to_string(), 10, 2)));
    app.lock().unwrap().push_element(Box::new(block));
    app.lock().unwrap().redraw();
    while app.lock().unwrap().state == 1 {
        
    };
}