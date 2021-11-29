fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[allow(non_snake_case)]
fn main() {
    let T = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..T {
        let cost = stdin_line().trim().parse::<usize>().unwrap();
        let line = stdin_line();
        let moneys_iter = line.split_whitespace();

        let values = [1, 2, 5, 10, 20, 50];
        let mut moneys = [0; 6]; // 1 2 5 10 20 50
        for (i, money) in moneys_iter.enumerate() {
            moneys[i] = money.parse::<usize>().unwrap();
        }

        let mut balance = 0;
        for i in 0..6 {
            balance += moneys[i] * values[i];
        }

        if balance >= cost {
            println!("ANO")
        } else {
            println!("NE")
        }
    }
}
