fn main() {
    twelves_days_of_xmas();
}

fn twelves_days_of_xmas() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    // We use days.iter() to be able to go for every element of the array
    // Then we use enumerate to know what index of that element is
    // Ex: (0, "first"), (1, "second"), etc.
    for (day_num, day) in days.iter().enumerate() {
        println!("For the {} day of Christmas my true love sent to me", day);

        for gift_day in (1..day_num + 1).rev() {
            // if is not the first day (beginning of the song) and we are in the first gift given
            // we add an "and " in the lyrics
            if gift_day == 1 && day_num != 1 {
                print!("and ");
            }
            if gift_day == 1 {
                println!("a Patridge in a Pear Tree");
            } else if gift_day == 2 {
                println!("Two Turtle Doves");
            } else if gift_day == 3 {
                println!("Three French Hens");
            } else if gift_day == 4 {
                println!("Four Calling Birds");
            } else if gift_day == 5 {
                println!("Five Golden Rings");
            } else if gift_day == 6 {
                println!("Six Geese a Laying");
            } else if gift_day == 7 {
                println!("Seven Swans a Swimming");
            } else if gift_day == 8 {
                println!("Eight Maids a Milking");
            } else if gift_day == 9 {
                println!("Nine Ladies Dancing");
            } else if gift_day == 10 {
                println!("Ten Lords a Leaping");
            } else if gift_day == 11 {
                println!("Eleven Pipers Piping");
            } else if gift_day == 12 {
                println!("Twelve Drummers Drumming");
            }
        }

        println!("\n");
    }
}
