fn main() {
    let days = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelvth"];
    let numbers = ["a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    let presents = [
        "Partridge in a Pear Tree",
        "Turtle Doves",
        "French Hens",
        "Gold Rings",
        "Calling Birds",
        "Geese a-Laying",
        "Swans a-Swimming",
        "Maids a-Milking",
        "Ladies Dancing",
        "Lords a-Leaping",
        "Pipers Piping",
        "Drummers Drumming"
    ];
    assert!(days.len() == 12);
    assert!(numbers.len() == 12);
    assert!(presents.len() == 12);

    for d in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[d]);
        for dd in (0..d+1).rev() {
            if dd != 0 {
                println!("{} {},", numbers[dd], presents[dd]);
            } else {
                if d != 0 {
                    print!("and ");
                }
                println!("{} {}.", numbers[dd], presents[dd]);
            }
        }
        println!("");
    }
}
