use std::io;
fn main() {
    println!("Fibonacci sequence!");
    println!("Select how many numbers you want to be displayed from the fibonacci sequence");

    loop {
        let mut index: String = String::new();
        io::stdin().read_line(&mut index).expect("Couldn't read the line");
        let index: u32 = match index.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Enter a valid positive number: ");
                continue;
            }
        };

        fibonacci(index);
        break;
    }
}

fn fibonacci(index: u32) {
    // fibonacci sequence is [1,1,2,3,5,8...]
    // we will make the sequence until we reach the index given
    let mut prev_num = 1;
    let mut current_num = 1;
    let mut counter = 0;

    print!("[");
    while counter != index {
        if counter == 0 || counter == 1 {
            print!("1");
        } else {
            let next_i = prev_num + current_num;
            prev_num = current_num;
            current_num = next_i;
            print!("{current_num}");
        }

        if counter + 1 != index {
            print!(", ");
        }
        counter += 1;
    }
    println!("]");
}
