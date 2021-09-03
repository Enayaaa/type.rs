extern crate pancurses;
extern crate rand;
extern crate textplots;
mod canvas;
mod formulas;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

use crate::{canvas::Canvas, formulas::gross_wpm};
use pancurses::*;
use rand::Rng;
use textplots::*;

const CANVAS_WIDTH: i32 = 40;
const CANVAS_HEIGHT: i32 = 5;

fn border(
    win: &Window,
    top: char,
    bottom: char,
    left: char,
    right: char,
    upper_left: char,
    upper_right: char,
    lower_left: char,
    lower_right: char,
) {
    win.mvprintw(0, 0, upper_left.to_string());
    while win.get_cur_x() < win.get_max_x() - 1 {
        win.printw(top.to_string());
    }
    win.printw(upper_right.to_string());
    while win.get_cur_y() < win.get_max_y() - 1 {
        win.printw(left.to_string());
        win.mvprintw(win.get_cur_y(), win.get_max_x() - 1, right.to_string());
    }
    win.printw(lower_left.to_string());
    while win.get_cur_x() < win.get_max_x() - 1 {
        win.printw(bottom.to_string());
    }
    win.printw(lower_right.to_string());
}

fn display_result(win: &Window, data: &Vec<(f32, f32)>, duration: Duration) {
    nl();
    raw();
    cbreak();

    let ymax = f32::INFINITY;
    let ymin = f32::NEG_INFINITY;
    let xmin = 0.0;
    let xmax = duration.as_secs_f32();
    let width = 40;
    let height = 32;

    let y = Shape::Lines(&data[..]);
    let mut z = Chart::new(width, height, xmin, xmax);
    let x = z.lineplot(&y);

    x.axis();
    x.figures();

    let frame = x.frame();
    let rows = frame.split('\n').count();
    for (i, row) in frame.split('\n').enumerate() {
        if i == 0 {
            win.printw(&format!("{0} {1:.1}\n", row, ymax));
        } else if i == (rows - 1) {
            win.printw(&format!("{0} {1:.1}\n", row, ymin));
        } else {
            win.printw(&format!("{}\n", row));
        }
    }

    win.printw(&format!(
        "{0: <width$.1}{1:.1}\n",
        xmin,
        xmax,
        width = (width as usize) / 2 - 3
    ));

    win.refresh();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // main text_win containing whole screen
    def_shell_mode();
    let stdscr = initscr();
    def_prog_mode();
    curs_set(0);
    start_color();
    // canvas for containing the text
    let begin_y = (stdscr.get_max_y() - CANVAS_HEIGHT) / 2;
    let begin_x = (stdscr.get_max_x() - CANVAS_WIDTH) / 2;
    let text_win = stdscr
        .subwin(CANVAS_HEIGHT, CANVAS_WIDTH, begin_y, begin_x)
        .unwrap();
    let input_box = stdscr
        .subwin(3, CANVAS_WIDTH, begin_y + text_win.get_max_y(), begin_x)
        .unwrap();
    let input_win = stdscr
        .subwin(1, CANVAS_WIDTH - 2, input_box.get_beg_y() + 1, begin_x + 1)
        .unwrap();
    let state_win = stdscr
        .subwin(1, CANVAS_WIDTH, begin_y - 3, begin_x)
        .unwrap();
    let mut canvas = Canvas::new(text_win, input_win, state_win);
    canvas.input_win.keypad(true);

    loop {
        let english = [
            "the", "be", "of", "and", "a", "to", "in", "he", "have", "it", "that", "for", "they",
            "I", "with", "as", "not", "on", "she", "at", "by", "this", "we", "you", "do", "but",
            "from", "or", "which", "one", "would", "all", "will", "there", "say", "who", "make",
            "when", "can", "more", "if", "no", "man", "out", "other", "so", "what", "time", "up",
            "go", "about", "than", "into", "could", "state", "only", "new", "year", "some", "take",
            "come", "these", "know", "see", "use", "get", "like", "then", "first", "any", "work",
            "now", "may", "such", "give", "over", "think", "most", "even", "find", "day", "also",
            "after", "way", "many", "must", "look", "before", "great", "back", "through", "long",
            "where", "much", "should", "well", "people", "down", "own", "just", "because", "good",
            "each", "those", "feel", "seem", "how", "high", "too", "place", "little", "world",
            "very", "still", "nation", "hand", "old", "life", "tell", "write", "become", "here",
            "show", "house", "both", "between", "need", "mean", "call", "develop", "under", "last",
            "right", "move", "thing", "general", "school", "never", "same", "another", "begin",
            "while", "number", "part", "turn", "real", "leave", "might", "want", "point", "form",
            "off", "child", "few", "small", "since", "against", "ask", "late", "home", "interest",
            "large", "person", "end", "open", "public", "follow", "during", "present", "without",
            "again", "hold", "govern", "around", "possible", "head", "consider", "word", "program",
            "problem", "however", "lead", "system", "set", "order", "eye", "plan", "run", "keep",
            "face", "fact", "group", "play", "stand", "increase", "early", "course", "change",
            "help", "line",
        ];
        let mut rng = rand::thread_rng();
        let mut output = String::new();
        while output.chars().count() < 180 {
            //if let Ok(mut lines) = read_lines("./src/lib/english.txt") {
            let x: usize = rng.gen_range(0..english.len() - 1);
            //if let Some(line) = lines.nth(x) {
            //    output.push_str(line.unwrap().trim());
            //    output.push(' ');
            //}
            output.push_str(english[x]);
            output.push(' ');
            //}
        }

        canvas.text = output;
        stdscr.erase();
        canvas.input = String::from("");

        nl();
        noecho();
        stdscr.keypad(true);

        border(&input_box, '─', '─', '│', '│', '╭', '╮', '╰', '╯');
        stdscr.refresh();

        let (duration, data) = canvas.run_test();
        stdscr.erase();
        stdscr.refresh();

        stdscr.attron(A_REVERSE);
        stdscr.mvprintw(
            5,
            (stdscr.get_max_x() - 10) / 2,
            &format!(" {} WPM ", gross_wpm(canvas.text.chars().count(), duration)),
        );
        stdscr.attroff(A_REVERSE);
        stdscr.printw(" ლ(ಠ益ಠლ)");

        let result_win = stdscr
            .subwin(
                32,
                40,
                (stdscr.get_max_y() - 32) / 2,
                (stdscr.get_max_x() - 40) / 2,
            )
            .unwrap();
        result_win.refresh();

        display_result(&result_win, &data, duration);

        stdscr.refresh();
        let input = stdscr.getch();
        if input == Some(Input::Character('q')) {
            break;
        }
    }

    endwin();
}
