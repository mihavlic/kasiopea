#![feature(label_break_value)]
 
fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[allow(non_snake_case)]
fn main() {
    let T = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..T {
        let line = stdin_line();
        let mut numbers = line.split_whitespace();

        let R = numbers.next().unwrap().parse::<usize>().unwrap();
        let S = numbers.next().unwrap().parse::<usize>().unwrap();
        let _K = numbers.next().unwrap().parse::<usize>().unwrap();

        let mut grid = vec![0; R * S].into_boxed_slice();

        for y in 0..R {
            let line = stdin_line();
            let mut numbers = line.split_whitespace();

            for x in 0..S {
                grid[y*S + x] = numbers.next().unwrap().parse::<i32>().unwrap();
            }
        }

        let mut y_start = R;
        // find the first non-empty row
        // FIXME better than linear search or move up into parsing stage, 
        // though it is linear in memory so should be very very fast and is not worth replacing
        for (i, value) in grid.iter().enumerate() {
            if *value > -1 {
                y_start = i / S;
                break;
            }
        }

        'y_loop: for y in y_start..R {
            let mut fill_value = 'search: {
                // search the row for a color, when the color is found, return and start filling the row with that color
                for x in 0..S {
                    let value = grid[y*S + x];
                    if value > -1 {
                        break 'search value;
                    }
                }
                // if there is no color, copy the previous row (the starting row is always selected to have a color so it cannot happen that an empty row is copied)
                for x in 0..S {
                    let value = grid[(y-1)*S + x];
                    grid[y*S + x] = value;
                }
                continue 'y_loop;
            };
            
            for x in 0..S {
                let value = &mut grid[y*S + x];
                
                // a new color, start filling with it
                if *value > -1 {
                    fill_value = *value;
                } else {
                    *value = fill_value;
                }
            }
        }

        // fix up the top which had potentially empty rows
        for y in 0..y_start {
            for x in 0..S {
                // load the values from the first valid row and copy it into the above empty ones, essentially extending the rectangles up
                // note y being replaced by y1 in the latter one
                grid[y*S + x] = grid[y_start*S + x];
            }
        }

        for y in 0..R {
            for x in 0..S {
                let value = grid[y*S + x];
                print!("{} ", value);
            }
            println!();
        }
    }
}
