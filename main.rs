use std::io;

fn get_rows () -> usize {
    let mut input = String::new();
    let n: usize = loop {
        println!("Enter a number between 1 and 9: ");
        io::stdin().read_line(&mut input).expect("Failed to Read");
        match input.trim().parse() {
            Ok(num) if (1..=9).contains(&num) => break num,
            _=> println!("Please enter a valid number between 1 and 9"),
    }
};
n
}

fn pascal(row: usize, col: usize) -> usize {
        if col == 0 || col == row {
            1
        } else {
            pascal(row-1, col-1) + pascal(row-1, col)
        }
}

// fn print_spaces(n: usize) {
//     let spaces = (n + n -1) / 2;
//     for i in 0..
// }

fn print_pascal_row(n: usize, row: usize) {
    for _ in 0..(n-row) {
        print!("  ");
    }
    for col in 0..=row {
        print!("{:4}", pascal(row, col));
    }
    println!();
}

fn main() {
    let n: usize = get_rows();
    for row in 0..n {
        print_pascal_row(n, row);
    }
}
