extern crate pancurses;
extern crate textwrap;

use crate::formulas::*;
use pancurses::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use textwrap::*;

const CORRECT_CHAR: u8 = 1;
const INCORRECT_CHAR: u8 = 2;
const RED_BACKGROUD: u8 = 3;

pub struct Canvas {
    pub text_win: Window,
    pub input_win: Window,
    pub state_win: Window,
    pub input: String,
    pub text: String,
    word_idx: usize,
}

impl Canvas {
    pub fn new(text_win: Window, input_win: Window, state_win: Window) -> Canvas {
        Canvas {
            text_win: text_win,
            input_win: input_win,
            state_win: state_win,
            input: String::new(),
            text: String::new(),
            word_idx: 0,
        }
    }

    pub fn get_words(&self) -> std::str::SplitWhitespace<'_> {
        self.text.split_whitespace()
    }

    fn get_char_index(&self) -> usize {
        let words = self
            .get_words()
            .take(self.word_idx)
            .map(|x| x.chars().count())
            .sum::<usize>();
        let spaces = self.word_idx;
        let input = self.input.chars().count();
        words + input + spaces
    }

    fn display_text(&self) {
        self.text_win.erase();

        let mut attr = Attributes::new();
        attr.set_bold(true);
        attr.set_color_pair(ColorPair(CORRECT_CHAR));

        let mut wrong_attr = Attributes::new();
        wrong_attr.set_bold(true);
        wrong_attr.set_color_pair(ColorPair(INCORRECT_CHAR));

        // Word wrap text for printing to the terminal
        let wrapped = fill(&self.text, self.text_win.get_max_x() as usize);

        // print text with correct attributes
        let input_len = self.input.chars().count();
        let char_idx = self.get_char_index();
        let done_until = char_idx - input_len;
        let correct_in_word = self
            .get_words()
            .nth(self.word_idx)
            .unwrap()
            .chars()
            .enumerate()
            .take_while(|&(i, c)| c == self.input.chars().nth(i).unwrap_or(0 as char))
            .count();
        // draw the correct words so far
        self.text_win.attron(attr);
        self.text_win
            .printw(&wrapped.chars().take(done_until).collect::<String>());
        // draw the correct chars in current word
        self.text_win.printw(
            &wrapped
                .chars()
                .skip(done_until)
                .take(correct_in_word)
                .collect::<String>(),
        );
        self.text_win.attroff(attr);
        // draw the incorrect chars in current word
        self.text_win.attron(wrong_attr);
        self.text_win.printw(
            &wrapped
                .chars()
                .skip(done_until + correct_in_word)
                .take(input_len - correct_in_word)
                .collect::<String>(),
        );
        self.text_win.attroff(wrong_attr);
        // draw the rest of the words not yet done
        self.text_win
            .printw(&wrapped.chars().skip(char_idx).collect::<String>());
    }

    fn display_input(&self) {
        if self.word_idx < self.get_words().count() {
            let is_ok = self
                .get_words()
                .nth(self.word_idx)
                .unwrap()
                .chars()
                .take(self.input.chars().count())
                .collect::<String>()
                == self.input;
            if self.input.chars().count()
                > self.get_words().nth(self.word_idx).unwrap().chars().count()
                || !is_ok
            {
                self.input_win.bkgd(COLOR_PAIR(RED_BACKGROUD as u32));
            } else {
                self.input_win.bkgd(COLOR_PAIR(0));
            }
        } else {
            self.input_win.bkgd(COLOR_PAIR(0));
        }
        self.input_win.mvprintw(0, 0, &self.input);
        self.input_win.printw("▂");
        //self.input_win.mvprintw(0, 10, "( ͡° ͜ʖ ͡°)");
    }

    fn display_state(&self, state: String) {
        let x = (self.state_win.get_max_x() - state.chars().count() as i32) / 2;
        let y = 0;
        self.state_win.mvprintw(y, x, &state);
        self.state_win.refresh();
    }

    pub fn run_test(&mut self) -> (Duration, Vec<(f32, f32)>) {
        use_default_colors();
        curs_set(0);
        self.input_win.nodelay(true);
        noecho();
        nonl();

        init_pair(CORRECT_CHAR as i16, COLOR_GREEN, -1);
        init_pair(INCORRECT_CHAR as i16, COLOR_WHITE, COLOR_RED);
        init_pair(RED_BACKGROUD as i16, COLOR_WHITE, COLOR_RED);

        self.display_text();
        self.text_win.refresh();
        self.input_win.mv(0, 0);

        let mut data: Vec<(f32, f32)> = Vec::new();
        let mut data_timer = Instant::now();

        let timer = Instant::now();
        loop {
            match self.input_win.getch() {
                Some(Input::Character('\x0D')) => (),
                Some(Input::Character(' ')) => {
                    if self.word_idx < self.get_words().count()
                        && self.input == self.get_words().nth(self.word_idx).unwrap()
                    {
                        self.input.clear();
                        self.input_win.erase();
                        self.word_idx += 1;
                    } else {
                        self.input.push(' ');
                    }
                }
                // Ctrl + backspace pressed
                Some(Input::Character('\x08')) => {
                    let word_len = self
                        .input
                        .split_whitespace()
                        .last()
                        .unwrap_or("")
                        .chars()
                        .count();
                    let spaces = self.input.chars().count() - self.input.trim_end().chars().count();
                    for _ in 0..word_len + spaces {
                        self.input.pop();
                    }
                    self.input_win.erase();
                }
                // handle backspace button
                Some(Input::KeyBackspace) => {
                    let _ = self.input.pop();
                    self.input_win.erase();
                }
                // normal characters
                Some(Input::Character(c)) => {
                    self.input.push(c);
                }
                // delete key quits the program
                Some(Input::KeyDC) => break,
                Some(_) => (),
                None => (),
            }

            self.display_state(format!("{}", timer.elapsed().as_secs()));
            self.display_input();
            self.display_text();
            self.text_win.refresh();

            if self.word_idx == self.get_words().count() - 1
                && self.get_words().nth(self.word_idx).unwrap() == self.input
            {
                self.input_win.erase();
                self.input_win.refresh();
                break;
            }

            if data_timer.elapsed() >= Duration::from_secs(1) {
                let wpm = gross_wpm(self.get_char_index(), timer.elapsed());
                data.push((timer.elapsed().as_secs_f32(), wpm as f32));
                data_timer = Instant::now();
            }

            sleep(Duration::new(0, 17000));
        }
        nl();
        echo();
        (timer.elapsed(), data)
    }
}
