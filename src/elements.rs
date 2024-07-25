use crate::{Window, App};
use std::sync::{Arc, Mutex};
use std::cmp;
use std::thread;

type Elem = dyn Element + Send;
type ElemStatic = dyn Element + Send + 'static;
type AppRef = Arc<Mutex<App>>;

pub trait Element {
    fn show(self: &Self, wnd: &mut Window, min_r: i16, max_r: i16, min_c: i16, max_c: i16);
    fn command_controller(self: & Self, text: &String, app: AppRef) {}
    fn get_id(self: &Self) -> &String;
    fn into_container(self: &mut Self) -> Option<Box<&mut dyn Container>> {
        None
    }
    fn r(self: &mut Self) -> &mut i16;
    fn c(self: &mut Self) -> &mut i16;
}

pub struct Text {
    text: String,
    r: i16,
    c: i16,
    id: String,
}

impl Text {
    pub fn new(text: &str, r: i16, c: i16, id: &str) -> Self{
        Text {
            text: text.to_string(),
            r, c,
            id: id.to_string()
        }
    }

    pub fn inbox(text: &str, r: i16, c: i16, id: &str) -> Box<Self>{
        Box::new(Self::new(text, r, c, id))
    }
}

impl Element for Text {
    fn show(self: &Self, wnd: &mut Window, min_r: i16, max_r: i16, min_c: i16, max_c: i16) {
        let r = self.r + min_r;
        let c =self. c + min_c;
        if r >= max_r {return;}
        for i in c..cmp::min(max_c, c + self.text.chars().count() as i16) {
            let rus: usize = r as usize;
            let ius: usize = i as usize;
            wnd.grid[rus][ius] = self.text.chars().nth((i - c) as usize).unwrap();
        }
    }

    fn get_id(self: &Self) -> &String {
        return &self.id;
    }

    fn r(self: &mut Self) -> &mut i16 {
        &mut self.r
    }

    fn c(self: &mut Self) -> &mut i16 {
        &mut self.c
    }
}

pub trait Container {
    fn get_elements(self: &mut Self) -> &mut Vec<Box<Elem>>;
    fn push_element(self: &mut Self, elem: Box<ElemStatic>) {
        self.get_elements().push(elem);
    }

    fn pop_element(self: &mut Self) -> Option<Box<Elem>> {
        self.get_elements().pop()
    }

    fn remove_element(self: &mut Self, index: usize) -> Option<Box<Elem>> {
        if index < self.get_elements().len() {
            Some(self.get_elements().remove(index))
        } else {
            None
        }
    }

    fn remove_element_by_id(self: &mut Self, id: &str) -> Option<Box<Elem>> {
        let mut i = 0;
        for elem in self.get_elements() {
            if elem.get_id().as_str() == id {
                break;
            }
            i += 1;
        }
        if i == self.get_elements().len() {
            None
        } else {
            Some(self.get_elements().remove(i))
        }
    }

    fn insert_element(self: &mut Self, elem: Box<ElemStatic>, index: usize) {
        self.get_elements().insert(index, elem);
    }

    fn get_element_by_id(self: &mut Self, id: &str) -> Option<&mut Box<Elem>> {
        let mut i = 0;
        for elem in self.get_elements() {
            if elem.get_id().as_str() == id {
                break;
            }
            i += 1;
        }
        if i == self.get_elements().len() {
            None
        } else {
            Some(&mut self.get_elements()[i])
        }
    }
}

pub enum Border {
    Slim,
    Bold,
}

pub struct Button {
    border: Border,
    padding: usize,
    command: String,
    on_click: Arc<dyn Fn(AppRef) + 'static + Send + Sync>,
    text: String,
    id: String,
    r: i16,
    c: i16,
}

impl Button {
    pub fn new(command: String, on_click: Arc<dyn Fn(AppRef) + 'static + Send + Sync>, text: &str, border: Border, padding: usize, id: String, r: i16, c: i16) -> Button {
        Button {
            command, on_click, 
            text: text.to_string(),
            border, padding, id, r, c
        }
    }

    pub fn inbox(command: String, on_click: Arc<dyn Fn(AppRef) + 'static + Send + Sync>, text: &str, border: Border, padding: usize, id: String, r: i16, c: i16) -> Box<Self>{
        Box::new(Self::new(command, on_click, text, border, padding, id, r, c))
    }
}

impl Element for Button {
    fn command_controller(self: &Self, text: &String, app: AppRef) {
        if *text == self.command {
            let on_click = self.on_click.clone();
            let app = app.clone();
            thread::spawn(move || {
                (*on_click)(app);
            });
        }
    }

    fn show(self: &Self, wnd: &mut Window, min_r: i16, max_r: i16, min_c: i16, max_c: i16) {
        let r = self.r + min_r;
        let c = self.c + min_c;
        let len: i16 = self.text.len() as i16;
        let padding: i16 = self.padding as i16;
        match self.border {
            Border::Slim => {
                if r > min_r + padding {
                    wnd.show_string(&String::from("╭".to_string() + "─".repeat((len + padding * 2) as usize).as_str() + "╮"), r - min_r - 1 - padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                }
                for r in (r - padding)..r {
                    if r >= min_r {
                        wnd.show_string(&String::from("│".to_string() + " ".repeat((len + padding * 2) as usize).as_str() + "│"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                    }
                }
                wnd.show_string(&String::from("│".to_string() + " ".repeat(padding as usize).as_str() + self.text.as_str() + " ".repeat(padding as usize).as_str() + "│"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                for r in (r + 1)..(r + 1 + padding) {
                    wnd.show_string(&String::from("│".to_string() + " ".repeat((len + padding * 2) as usize).as_str() + "│"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                }
                wnd.show_string(&String::from("╰".to_string() + "─".repeat((len + padding * 2) as usize).as_str() + "╯"), r - min_r + 1 + padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
            }
            Border::Bold => {
                if r > min_r + padding {
                    wnd.show_string(&String::from("┏".to_string() + "━".repeat((len + padding * 2) as usize).as_str() + "┓"), r - min_r - 1 - padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);}
                for r in (r - padding)..r {
                    if r >= min_r {
                        wnd.show_string(&String::from("┃".to_string() + " ".repeat((len + padding * 2) as usize).as_str() + "┃"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                    }
                }
                wnd.show_string(&String::from("┃".to_string() + " ".repeat(padding as usize).as_str() + self.text.as_str() + " ".repeat(padding as usize).as_str() + "┃"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                for r in (r + 1)..(r + 1 + padding) {
                    wnd.show_string(&String::from("┃".to_string() + " ".repeat((len + padding * 2) as usize).as_str() + "┃"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                }
                wnd.show_string(&String::from("┗".to_string() + "━".repeat((len + padding * 2) as usize).as_str() + "┛"), r - min_r + 1 + padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
            }
        }
    }

    fn get_id(self: &Self) -> &String {
        &self.id
    }

    fn r(self: &mut Self) -> &mut i16 {
        &mut self.r
    }

    fn c(self: &mut Self) -> &mut i16 {
        &mut self.c
    }
}

pub struct Block {
    width: usize,
    height: usize,
    border: Border,
    padding: usize,
    name: String,
    c: i16,
    r: i16,
    id: String,
    elements: Vec<(Box<Elem>)>,
}

impl Block {
    pub fn new(width: usize, height: usize, border: Border, padding: usize, name: &str, id: &str, r: i16, c: i16) -> Self {
        Block {
            width, height, border, padding,
            name: name.to_string(),
            id: id.to_string(),
            elements: Vec::new(),
            r, c
        }
    }

    pub fn inbox(width: usize, height: usize, border: Border, padding: usize, name: &str, id: &str, r: i16, c: i16) -> Box<Self>{
        Box::new(Self::new(width, height, border, padding, name, id, r, c))
    }
}

impl Container for Block {
    fn get_elements(self: &mut Self) -> &mut Vec<Box<Elem>> {
        &mut self.elements
    }
}

impl Element for Block {
    fn show(self: &Self, wnd: &mut Window, min_r: i16, max_r: i16, min_c: i16, max_c: i16) {
        let width = self.width as i16;
        let height = self.height as i16;
        let padding: i16 = self.padding as i16;
        let r = self.r + min_r;
        let c = self.c + min_c;
        match self.border {
            Border::Slim => {
                if r > min_r + padding {
                    wnd.show_string(&String::from("╭".to_string() + "─".repeat((width + padding * 2) as usize).as_str() + "╮"), r - min_r - 1 - padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);}
                for r in (r - padding)..(r + height + padding)  {
                    if r >= min_r {
                        wnd.show_string(&String::from("│".to_string() + " ".repeat((width + padding * 2) as usize).as_str() + "│"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                    }
                }
                wnd.show_string(&String::from("╰".to_string() + "─".repeat((width + padding * 2) as usize).as_str() + "╯"), r - min_r + height + padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
            }
            Border::Bold => {
                if r > min_r + padding {
                    wnd.show_string(&String::from("┏".to_string() + "━".repeat((width + padding * 2) as usize).as_str() + "┓"), r - min_r - 1 - padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                }
                for r in (r - padding)..(r + height + padding)  {
                    if r >= min_r {
                        wnd.show_string(&String::from("┃".to_string() + " ".repeat((width + padding * 2) as usize).as_str() + "┃"), r - min_r, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
                    }
                }
                wnd.show_string(&String::from("┗".to_string() + "━".repeat((width + padding * 2) as usize).as_str() + "┛"), r - min_r + height + padding, c - min_c - 1 - padding, min_r, max_r, min_c, max_c);
            }
        }
        wnd.show_string(&self.name, 0, 0, r - padding - 1, r + height + padding + 1, c - padding, c + width + padding);

        
        for elem in &self.elements {
            elem.show(wnd, r, r + height, c, c + width);
        }        
    }

    fn command_controller(self: &Self, text: &String, app: AppRef) {
        let elements= &self.elements;
        for elem in elements {
            elem.command_controller(text, app.clone());
        }
    }

    fn get_id(self: &Self) -> &String {
        &self.id
    }

    fn into_container(self: &mut Self) -> Option<Box<&mut dyn Container>> {
        Some(Box::new(self))
    }

    fn r(self: &mut Self) -> &mut i16 {
        &mut self.r
    }

    fn c(self: &mut Self) -> &mut i16 {
        &mut self.c
    }
}