extern crate pancurses;
mod canvas;

use pancurses::*;
use crate::canvas::Canvas;

const CANVAS_WIDTH: i32 = 40;
const CANVAS_HEIGHT: i32 = 5;


fn border(win: &Window, top: char, bottom: char, left: char, right: char,
          upper_left: char, upper_right: char, lower_left: char, lower_right: char) {
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


fn main() {
    // main window containing whole screen
    let stdscr = initscr();
    curs_set(0);
    
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
    
    start_color();
    canvas.input_win.keypad(true);

    let output = "This is a very long text to test line breaks and such \
        so just bear with me here mate and we'll figure this out. Soon enough \
        I hope but you never know with this things, you know. ";
    

    for word in output.split_whitespace() {
        canvas.words.push(word.to_string());
    }

    border(&input_box, '─', '─', '│', '│', '╭', '╮', '╰', '╯');
    stdscr.refresh();

    let time = canvas.run_test();

    stdscr.attron(A_REVERSE);
    stdscr.mvprintw(5, (stdscr.get_max_x() - 10) / 2,
            &format!(" {} WPM ", ((output.chars().count() as f32 / 5.1f32) / (time.as_secs() as f32 / 60f32)) as usize));
    stdscr.refresh();
    stdscr.getch();
    endwin();
}
