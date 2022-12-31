extern crate colored;

use colored::Colorize;

const TREE: &str = "\
|-----------------------------------------------|\n\
|                                               |\n\
|                                               |\n\
|                       O                       |\n\
|                      OOO                      |\n\
|                      ~O~                      |\n\
|                     ~~~~~                     |\n\
|                    O~~~~~~                    |\n\
|                   ~~OO~~OO~                   |\n\
|                  ~~~~~OO~~~~                  |\n\
|                 ~~~~~~~~~~~~~                 |\n\
|                OO~~~~~~~~~~~~~                |\n\
|               ~~~O~~~~~><~~~~~O               |\n\
|              ~~~~~O~~~~~~~~~~O~~              |\n\
|             ~~~~~~~OO~~~~~~~O~~~~             |\n\
|            ~~~~><~~~~OO~~~OO~~~~~~            |\n\
|           ~~~~~~~~~~~~~OOO~~~~~~~~~           |\n\
|          ~~~~~~~~~~~~~~~~~~~~~~~~~~~          |\n\
|         ~~~~~~~~~~~~~~~~~~~~~~~~~~OO~         |\n\
|        O~~~~><~~~~~~~~~><~~~~~~~OO~~~~        |\n\
|       ~~OO~~~~~~~~~~~~~~~~~~~~OO~~~~~~~       |\n\
|      ~~~~~OO~~~~~~~~~~~~~~~OOO~~~~~~~~~~      |\n\
|     ~~~~~~~~OO~~~~~~~~~~OOO~~~~~~~~~~~~~~     |\n\
|    ~~~><~~~~~~OOO~~~~OOO~~~~~~~~~~~~~~~~~~    |\n\
|   ~~~~~~~~~~~~~~~OOOO~~~~~~~~~~><~~~~~~~~~~   |\n\
|  ~~~~~~~~~~~~~~OO~~~OOO~~~~~~~~~~~~~~~~~~~~~  |\n\
|                     #####                     |\n\
|                     #####                     |\n\
|                     #####                     |\n\
|                                               |\n\
|-----------------------------------------------|\n\
";

enum Type {
    Frame,
    Tree,
    Decoration,
    Led
}

fn char_type(c: char) -> Type {
    match c {
        '|' | '-' | ' ' => Type::Frame,
        '~' | '#' => Type::Tree,
        '>' | '<' => Type::Decoration,
        'O' => Type::Led,
        _ => unreachable!()
    }
}

fn show_tree() {
    for c in TREE.chars() {
        if c == '\n' {
            println!();
        } else {
            let fmt = format!("{}", c);
            print!("{}",
                match char_type(c) {
                    Type::Frame => fmt.bold(),
                    Type::Tree => fmt.green(),
                    Type::Decoration => fmt.red(),
                    Type::Led => fmt.blue(),
                });
        }
    }
}

fn main() {
    show_tree();
}
