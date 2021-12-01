struct Number {
    // true positive, false negative
    sign: bool,
    digits: Vec<u8>
}

fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

fn stdin_number_digits(digits_count: usize) -> Vec<i8> {
    let mut digits = vec![0; digits_count];

    for (i, char) in stdin_line().chars().enumerate() {
        let digit = match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => break,
        };

        digits[i] = digit;
    }

    digits
}

fn substract(a_digits: &Number, b_digits: &Number) -> Vec<i8> {
    let mut result = Vec::new();
    result.resize(a_digits.len(), 0);

    let mut underflow = 0;
    for i in (0..result.len()).rev() {
        let a = a_digits[i];
        let b = b_digits[i];

        let mut c = a - b - underflow;

        // this doesnt work at all
        if c.is_negative() {
            underflow = 1;
            c += 10;
        } else {
            underflow = 0;
        }

        result[i] = c;
    }

    result
}

fn pretty_print_number(num: &Vec<i8>) {
    for digit in num {
        if *digit == 0 {
            continue;
        }
        if digit.is_negative() {
            print!("-");
        } else {
            print!(" ");
        }
        break;
    }
    for digit in num {
        print!("{}", digit.abs());
    }
    println!();
}

#[allow(non_snake_case)]
fn main() {
    let T = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..T {

        let N = stdin_line().trim().parse::<usize>().unwrap();
        
        let A = stdin_number_digits(N);
        let B = stdin_number_digits(N);
        let _C = stdin_number_digits(N);
        
        let honk = substract(&A, &B);
        
        pretty_print_number(&A);
        pretty_print_number(&B);
        pretty_print_number(&honk);

    }
}