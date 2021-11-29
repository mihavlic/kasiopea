fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[allow(non_snake_case)]
fn main() {
    let T = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..T {
        let N = stdin_line().trim().parse::<usize>().unwrap();

        let line = stdin_line();
        let grade_dates = line.split_whitespace().map(|g| g.parse::<usize>().unwrap());

        let mut sum = 0;
        for week_number in grade_dates {
            let grade = if week_number % 2 == 0 {
                2
            } else {
                3
            };

            sum += grade;
        }

        let final_grade = ((sum as f64) / (N as f64)).round() as usize;

        if final_grade == 2 {
            println!("ANO");
        } else {
            println!("NE");
        }
    }
}

