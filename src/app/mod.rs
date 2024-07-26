use std::thread::JoinHandle;
use std::{io, thread};
use std::io::{Write, stdout, Stdout};
use elements::*;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use std::sync::{Arc, Mutex};
use std::cmp;

type Elem = dyn Element + Send;
type AppRef = Arc<Mutex<App>>;

pub mod elements;
pub struct App {
    elements: Vec<Box<Elem>>,
    window: Window,
    stdout: Stdout,
    pub state: u32,
}

pub struct Window {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl Window {
    fn new(w: usize, h: usize) -> Window {
        Window {
            height: h,
            width: w,
            grid: vec![vec![' '; w.into()]; h.into()],
        }
    }
    fn clear(self: &mut Self) {
        self.grid = vec![vec![' '; self.width.into()]; self.height.into()];
    }
    fn show_string(self: &mut Self, str: &String, r: i16, c: i16, min_r: i16, max_r: i16, min_c: i16, max_c: i16) {
        let r = r + min_r;
        let c = c + min_c;
        if r >= max_r {return;}
        for i in c..cmp::min(max_c, c + str.chars().count() as i16) {
            let rus: usize = r as usize;
            let ius: usize = i as usize;
            self.grid[rus][ius] = str.chars().nth((i - c) as usize).unwrap();
        }
    }
}

impl App {
    pub fn new(command_controller: impl Fn(&String, AppRef) + Send + 'static) -> AppRef {
        let terminal_size = terminal::size().unwrap();
        let mut out = stdout();
        out.execute(EnterAlternateScreen).unwrap();
        let mut app = 
        Arc::new(Mutex::new(App {
            state: 1,
            elements: Vec::new(),
            stdout: out,
            window: Window::new(terminal_size.0.into() , (terminal_size.1 - 1).into()),
        }));
        //thread of command checkingButton │   
        {
            let app = app.clone();
            thread::spawn(move || {
                loop {
                    let mut text = String::new();
                    io::stdin().read_line(&mut text).unwrap();
                    //println!("{}:{}", text, text.chars().nth(4).unwrap());
                    match text.as_str() {
                        "quit\n" => {
                            app.lock().unwrap().state = 0;
                            return ;
                        },
                        _ => {
                            command_controller(&text, app.clone());
                            let elements= &mut app.lock().unwrap().elements;
                            for elem in elements {
                                elem.command_controller(&text, app.clone());
                            }
                        }
                    }
                    app.lock().unwrap().redraw();
                }
            });
        }
        app
    }

    fn get_text_to_print(grid: &Vec<Vec<char>>) -> String {
        let mut text = String::new();

        for line in grid {
            text.push_str(line.iter().collect::<String>().as_str());
            text.push('\n');
        }

        text.push_str("> $ ");

        text
    }

    pub fn redraw(self: &mut Self) {
        self.window.clear();
        let width = self.window.width as i16;
        let height = self.window.height as i16;
        for elem in &self.elements {
            elem.show(&mut self.window, 0, height, 0, width);
        }

        let text = App::get_text_to_print(&self.window.grid);

        let stdout = &mut self.stdout;
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.write_all(text.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    pub fn ininityloop(app: Arc<Mutex<Self>>) -> JoinHandle<()>{
        thread::spawn(move || while app.lock().unwrap().state == 1 {
            
        })
    }
}

impl elements::Container for App {
    fn get_elements(self: &mut Self) -> &mut Vec<Box<Elem>> {
        &mut self.elements
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.stdout.execute(LeaveAlternateScreen).unwrap();
    }
}