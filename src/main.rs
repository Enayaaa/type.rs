extern crate pancurses;
extern crate textplots;
mod canvas;
mod formulas;

use std::{result, time::Duration};

use crate::{canvas::Canvas, formulas::gross_wpm};
use pancurses::*;
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
    let width = 100;
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

fn main() {
    // main text_win containing whole screen
    def_shell_mode();
    let stdscr = initscr();
    def_prog_mode();
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

    canvas.text = String::from(output);

    nl();
    noecho();
    stdscr.keypad(true);
    // stdscr.printw(
    //     "
    //      ____  _  _  ____  ____     ____  ____
    //     (_  _)( \\/ )(  _ \\(  __)   (  _ \\/ ___)
    //       )(   )  /  ) __/ ) _)  _  )   /\\___ \\
    //      (__) (__/  (__)  (____)(_)(__\\_)(____/
    //         ",
    // );
    // stdscr.printw("\nPress tab to start test");
    // loop {
    //     match stdscr.getch() {
    //         Some(Input::Character('\t')) => break,
    //         Some(x) => {
    //             stdscr.printw(&format!("{:?}", x));
    //         }
    //         Some(_) => (),
    //         None => (),
    //     }
    // }

    border(&input_box, '─', '─', '│', '│', '╭', '╮', '╰', '╯');
    stdscr.refresh();

    let (duration, data) = canvas.run_test();
    stdscr.erase();
    stdscr.refresh();

    stdscr.attron(A_REVERSE);
    stdscr.mvprintw(
        5,
        (stdscr.get_max_x() - 10) / 2,
        &format!(" {} WPM ", gross_wpm(output.chars().count(), duration)),
    );
    stdscr.attroff(A_REVERSE);
    stdscr.printw(" ლ(ಠ益ಠლ)");

    let result_win = stdscr
        .subwin(
            32,
            100,
            (stdscr.get_max_y() - 32) / 2,
            (stdscr.get_max_x() - 100) / 2,
        )
        .unwrap();
    result_win.refresh();

    display_result(&result_win, &data, duration);

    stdscr.refresh();
    stdscr.getch();
    endwin();
}
