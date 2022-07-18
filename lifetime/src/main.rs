#![allow(dead_code)]
#![allow(unused_variables)]

struct Line {
    i: i32,
}

struct Graph<'a> {
    lines: Vec<&'a Line>,
    current: &'a Line,
}

fn test() {
    let line;
    let line1;
    let lines;

    {
        line = Line { i: 6 };
        line1 = Line { i: 7 };
        lines = vec![&line, &line1];
    }

    let g = &Graph {
        lines: lines,
        current: &line,
    };

    println!("{}", g.lines[0].i);
}

fn test1() {
    let x;
    let y = 5;
    x = &y;

    println!("{}", x);
}

fn main() {
    test();
}
