extern crate ncurses;

use ncurses::*;
use std::env;
use std::fs;

fn main() {

    let contents = fs::read_to_string("todos.txt")
        .expect("Something went wrong reading the file");
    let todo_itr = contents.split("\n");
    let mut todos: Vec<&str> = todo_itr.collect();

    let args: Vec<String> = env::args().collect();
    for i in args {
        if i == "-h" || i == "--help" {
            println!("No help to you");
            return;
        }
        if i == "-v" || i == "--version" {
            println!("Niyo Alpha");
            return;
        }
        if i == "-t" || i == "--todo" {
            return;
        }
    }


    initscr();
    addstr("Hello World!");
    refresh();
    getch();
    endwin();
}
