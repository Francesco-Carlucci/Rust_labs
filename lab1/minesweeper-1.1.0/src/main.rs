use minesweeper::annotate;

fn main() {
    let minefield = vec![
        " * ",
        " * ",
        "   ",
    ];
    println!("{:?}", annotate(&minefield));
}
