use colored::Colorize;
use ncurses;

pub const LED: char = 'O';
pub const TREE: &str = "\
|---------------------------------------------------|\n\
|                                                   |\n\
|                                                   |\n\
|                         O                         |\n\
|                        OOO                        |\n\
|                        ~O~                        |\n\
|                       ~~~~~                       |\n\
|                      O~~~~~~                      |\n\
|                     ~~OO~~OO~                     |\n\
|                    ~~~~~OO~~~~                    |\n\
|                   ~~~~~~~~~~~~~                   |\n\
|                  OO~~~~~~~~~~~~~                  |\n\
|                 ~~~O~~~~~><~~~~~O                 |\n\
|                ~~~~~O~~~~~~~~~~O~~                |\n\
|               ~~~~~~~OO~~~~~~~O~~~~               |\n\
|              ~~~~><~~~~OO~~~OO~~~~~~              |\n\
|             ~~~~~~~~~~~~~OOO~~~~~~~~~             |\n\
|            ~~~~~~~~~~~~~~~~~~~~~~~~~~~            |\n\
|           ~~~~~~~~~~~~~~~~~~~~~~~~~~OO~           |\n\
|          O~~~~><~~~~~~~~~><~~~~~~~OO~~~~          |\n\
|         ~~OO~~~~~~~~~~~~~~~~~~~~OO~~~~~~~         |\n\
|        ~~~~~OO~~~~~~~~~~~~~~~OOO~~~~~~~~~~        |\n\
|       ~~~~~~~~OO~~~~~~~~~~OOO~~~~~~~~~~~~~~       |\n\
|      ~~~><~~~~~~OOO~~~~OOO~~~~~~~~~~~~~~~~~~      |\n\
|     ~~~~~~~~~~~~~~~OOOO~~~~~~~~~~><~~~~~~~~~~     |\n\
|    ~~~~~~~~~~~~~OOO~~~OOO~~~~~~~~~~~~~~~~><~~~    |\n\
|   ~~~~~~~~~~~OOO~~~~~~~~~OO~~~~~~~~~~~~~~~~~~~~   |\n\
|                       #####                       |\n\
|                       #####                       |\n\
|                       #####                       |\n\
|                                                   |\n\
|                                                   |\n\
|---------------------------------------------------|\n\
";

pub trait UI {
    fn show(&mut self, groups: &Vec<Vec<u8>>);
}

pub struct CursesUI {
    offset: (i32, i32),
    colors: Vec<i16>,
}

impl CursesUI {
    const COLOR_PAIR_DEFAULT: i16 = 1;
    const COLOR_PAIR_TREE: i16 = 2;
    const COLOR_PAIR_DECORATION: i16 = 3;

    const COLOR_PAIR_BLACK: i16 = 4;
    const COLOR_PAIR_RED: i16 = 5;
    const COLOR_PAIR_GREEN: i16 = 6;
    const COLOR_PAIR_YELLOW: i16 = 7;
    const COLOR_PAIR_BLUE: i16 = 8;
    const COLOR_PAIR_MAGENTA: i16 = 9;
    const COLOR_PAIR_CYAN: i16 = 10;
    const COLOR_PAIR_WHITE: i16 = 11;

    pub fn new() -> Self {
        ncurses::initscr();
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        ncurses::start_color();

        let bg = ncurses::COLOR_BLACK;

        ncurses::init_pair(Self::COLOR_PAIR_DEFAULT, ncurses::COLOR_YELLOW, bg);
        ncurses::init_pair(Self::COLOR_PAIR_TREE, ncurses::COLOR_GREEN, bg);
        ncurses::init_pair(Self::COLOR_PAIR_DECORATION, ncurses::COLOR_RED, bg);

        ncurses::init_pair(Self::COLOR_PAIR_BLACK, ncurses::COLOR_BLACK, bg);
        ncurses::init_pair(Self::COLOR_PAIR_RED, ncurses::COLOR_RED, bg);
        ncurses::init_pair(Self::COLOR_PAIR_GREEN, ncurses::COLOR_GREEN, bg);
        ncurses::init_pair(Self::COLOR_PAIR_YELLOW, ncurses::COLOR_YELLOW, bg);
        ncurses::init_pair(Self::COLOR_PAIR_BLUE, ncurses::COLOR_BLUE, bg);
        ncurses::init_pair(Self::COLOR_PAIR_MAGENTA, ncurses::COLOR_MAGENTA, bg);
        ncurses::init_pair(Self::COLOR_PAIR_CYAN, ncurses::COLOR_CYAN, bg);
        ncurses::init_pair(Self::COLOR_PAIR_WHITE, ncurses::COLOR_WHITE, bg);

        let mut window = (0, 0);
        ncurses::getmaxyx(ncurses::stdscr(), &mut window.0, &mut window.1);

        let tree = (TREE.lines().count() as i32, TREE.lines().next().unwrap().len() as i32);

        Self {
            offset: ((window.0 - tree.0) / 2, (window.1 - tree.1) / 2),
            colors: vec![
                Self::COLOR_PAIR_BLACK,
                Self::COLOR_PAIR_RED,
                Self::COLOR_PAIR_GREEN,
                Self::COLOR_PAIR_YELLOW,
                Self::COLOR_PAIR_BLUE,
                Self::COLOR_PAIR_MAGENTA,
                Self::COLOR_PAIR_CYAN,
                Self::COLOR_PAIR_WHITE,
            ],
        }
    }
}

impl UI for CursesUI {
    fn show(&mut self, groups: &Vec<Vec<u8>>) {
        let mut color = groups
            .iter()
            .flatten()
            .chain(std::iter::repeat(&0))
            .map(|c| self.colors[*c as usize]);

        ncurses::mv(self.offset.0, self.offset.1);

        for c in TREE.chars() {
            let attr = ncurses::COLOR_PAIR(match c {
                '|' | '-' | ' ' => Self::COLOR_PAIR_DEFAULT,
                '~' | '#' => Self::COLOR_PAIR_TREE,
                '>' | '<' => Self::COLOR_PAIR_DECORATION,
                LED => color.next().unwrap(),

                '\n' => {
                    let mut y = 0;
                    ncurses::getyx(ncurses::stdscr(), &mut y, &mut 0);
                    ncurses::mv(y + 1, self.offset.1);
                    continue;
                }

                _ => unreachable!()
            });

            ncurses::attron(attr);
            ncurses::addch(c as ncurses::chtype);
            ncurses::attroff(attr);
        }

        ncurses::refresh();
        ncurses::mv(0, 0);
    }
}

impl Drop for CursesUI {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

pub struct SimpleUI {}

impl SimpleUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl UI for SimpleUI {
    fn show(&mut self, groups: &Vec<Vec<u8>>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut color = groups.iter().flatten().chain(std::iter::repeat(&0));

        let mut buf = String::new();
        for c in TREE.chars() {
            if c == '\n' {
                println!();
            } else {
                if ['~', '#', ' '].contains(&c) {
                    buf.push(c);
                    continue;
                } else {
                    print!("{}", format!("{}", buf).bright_green());
                    buf.clear();
                }

                let fmt = format!("{}", c);
                print!(
                    "{}",
                    match c {
                        '|' | '-' => fmt.bold(),
                        '>' | '<' => fmt.red(),
                        LED => {
                            match color.next() {
                                Some(0) => fmt.black(),
                                Some(1) => fmt.red(),
                                Some(2) => fmt.green(),
                                Some(3) => fmt.yellow(),
                                Some(4) => fmt.blue(),
                                Some(5) => fmt.magenta(),
                                Some(6) => fmt.cyan(),
                                Some(7) => fmt.white(),
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                );
            }
        }
    }
}

pub fn parse_groups() -> Vec<Vec<u8>> {
    let counts = TREE
        .lines()
        .map(|s| s.chars().fold(0, |acc, c| acc + (c == LED) as usize))
        .collect::<Vec<_>>();

    let mut groups: Vec<Vec<u8>> = vec![Vec::new()];
    for count in counts {
        if count == 0 && groups.last().unwrap().len() > 0 {
            groups.push(Vec::new());
        } else {
            groups
                .last_mut()
                .unwrap()
                .extend(std::iter::repeat(0).take(count));
        }
    }

    if groups.last().unwrap().is_empty() {
        groups.pop();
    }

    groups
}
