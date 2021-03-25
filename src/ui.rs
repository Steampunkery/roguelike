use std::cmp::max;
use std::iter::FromIterator;

const HORIZONTAL: &str = "━";
const VERTICAL: &str = "┃";
const TOP_LEFT: &str = "┏";
const TOP_RIGHT: &str = "┓";
const BOTTOM_LEFT: &str = "┗";
const BOTTOM_RIGHT: &str = "┛";

pub struct Menu {
    menu: Vec<String>,
    num_options: u8,
    width: u8,
    height: u8,
}

pub fn menu(title: &str, options: Vec<String>) -> Option<Menu> {
    if options.len() == 0 { return None }

    let options_max = options.iter().fold(&options[0], |acc, x| {
        if x.len() > acc.len() { return x }
        acc
    }).len();

    // 8 is 2 box characters + 2 padding characters + "(x) "
    let width = max(title.len(), options_max) + 8;
    // Plus 3 for title and bounds
    let height = options.len() + 3;

    let mut menu = vec![String::new(); options.len()];

    for (i, line) in menu.iter_mut().enumerate() {
        let content = format!("({}) {}", /* this is an ugly hack */ ((i+97) as u8) as char, options[i]);
        *line = format!("┃ {:<1$} ┃", content, width-4);
    }

    let mut top_line = vec![HORIZONTAL; width];
    top_line[0] = TOP_LEFT;

    let top_line_len = top_line.len()-1;
    top_line[top_line_len] = TOP_RIGHT;

    let title_line = format!("┃ {:<1$} ┃", title, width-4);

    let mut top_line_str = String::from_iter(top_line.clone());

    menu.splice(0..0, [top_line_str.clone(), title_line].iter().cloned());

    //reuse top_line as the bottom line
    top_line[0] = BOTTOM_LEFT;
    top_line[top_line_len]= BOTTOM_RIGHT;
    top_line_str = String::from_iter(top_line);

    menu.push(top_line_str);

    Some(Menu {
            menu,
            num_options: options.len() as u8,
            width: width as u8,
            height: height as u8,
    })
}