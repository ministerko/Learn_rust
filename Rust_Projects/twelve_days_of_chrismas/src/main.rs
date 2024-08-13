fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",

    ];

    for day in 0..12{
        println!("On the {} day of Christmas, my true love gave to me:" ,day + 1);

        for gift in (0..=day).rev(){
            if day > 0 && gift == 0 {
                print!("And ");
            }
            println!("{}", gifts[gift]);

        }
        println!(); // Print an empty line for separation
    }
}