extern crate pancurses;

use pancurses::*;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct Canvas {
    pub window: Window,
    pub input_win: Window,
    state_win: Window,
    pub input: String,
    pub words: Vec<String>,
    word_idx: usize,
}

impl Canvas {
    pub fn new(win: Window, input_win: Window, state_win: Window) -> Canvas {
        Canvas{
            window: win,
            input_win: input_win,
            state_win: state_win,
            input: String::new(),
            words: Vec::new(),
            word_idx: 0,
        }
    }

    fn display(&self) {
        let (mut y, mut x) = (0,0);
        
        self.window.erase();

        init_pair(1, COLOR_GREEN, COLOR_BLACK);
        let mut attr = Attributes::new();
        attr.set_bold(true);
        attr.set_color_pair(ColorPair(1));
        self.window.attron(attr);

        init_pair(2, COLOR_RED, COLOR_BLACK);
        let mut wrong_attr = Attributes::new();
        wrong_attr.set_bold(true);
        wrong_attr.set_color_pair(ColorPair(2));

        for (i, word) in self.words.iter().enumerate() {
            if x + word.chars().count() as i32 > self.window.get_max_x() {
                y += 1;
                x = 0;
            }

            if self.word_idx <= i {
                self.window.attroff(attr);
            } 

            if self.word_idx == i {
                for (j, c) in self.words[i].chars().enumerate() {
                    if j >= self.input.chars().count() {
                        self.window.attroff(wrong_attr);
                        self.window.attroff(attr);
                    } else if Some(c) == self.input.chars().nth(j) {
                        self.window.attroff(wrong_attr);
                        self.window.attron(attr);
                    } else {
                        self.window.attroff(attr);
                        self.window.attron(wrong_attr);
                    }
                    self.window.mvprintw(y,x+j as i32,c.to_string());
                }
            } else {
                self.window.mvprintw(y,x,&word);
            }

            x += word.chars().count() as i32 + 1;
        }
    }

    fn display_state(&self, state: String) {
        let x = (self.state_win.get_max_x() - state.chars().count() as i32) / 2;
        let y = 0;
        self.state_win.mvprintw(y, x, &state);
        self.state_win.refresh();
    }

    pub fn run_test(&mut self) -> Duration {
        curs_set(0);
        noecho();
        nonl();
        self.input_win.nodelay(true);
        self.display();
        self.window.refresh();
        self.input_win.mv(0,0);

        let timer = Instant::now();
    
        loop {
            match self.input_win.getch() {
                Some(Input::Character('\n')) => (),
                Some(Input::Character(' ')) => {
                    if self.word_idx < self.words.len() 
                        && self.input == self.words[self.word_idx]  {
                        self.word_idx += 1;
                        self.input.clear();
                        self.input_win.erase();
                    } else {
                        self.input.push(' ');
                    }
                }
                // Ctrl + delete pressed
                Some(Input::Character('\x07')) => {
                    let word_len = self.input.split_whitespace().last().unwrap_or("").chars().count();
                    let spaces = self.input.chars().count() - self.input.trim_end().chars().count();
                    for _ in 0..word_len + spaces { self.input.pop(); }
                    self.input_win.erase();
                }
                // normal characters
                Some(Input::Character(c)) => {
                        self.input.push(c);
                        //self.input_win.printw(&format!("{:?}", c));
                }
                // delete key quits the program
                Some(Input::KeyDC) => break,
                // handle backspace button
                Some(Input::KeyBackspace) => {
                    let _ = self.input.pop();
                    self.input_win.erase();
                }
                Some(input) => //(),
                {
                    self.input_win.printw(&format!("{:?}", input));
                }
                None => (),
            }

            init_pair(3, COLOR_WHITE, COLOR_RED);

            if self.word_idx < self.words.len() {
                let is_ok =  self.words[self.word_idx].find(&self.input);
                if self.input.len() > self.words[self.word_idx].len() 
                    || is_ok == None {
                    self.input_win.bkgd(COLOR_PAIR(3));
                } else {
                    self.input_win.bkgd(COLOR_PAIR(0)); 
                }
            } else {
                self.input_win.bkgd(COLOR_PAIR(0)); 
            }
            
            self.input_win.mvprintw(0, 0, &self.input);
            self.input_win.printw("_");
            self.display();
            self.display_state(format!("{}", timer.elapsed().as_secs()));
            if self.window.is_touched() { self.window.refresh(); } 

            if self.word_idx == self.words.len() - 1
                && self.words[self.word_idx] == self.input {
                break;
            }

            sleep(Duration::new(0,100000));
        }
        nl();
        echo();
        timer.elapsed()
    }
}

