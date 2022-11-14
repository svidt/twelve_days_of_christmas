fn main() {

    let dayth = [
        "First","Second", "Third",
        "Forth", "Fifth", "Sixth",
        "Seventh", "Eighth", "Nineth",
        "Tenth", "Eleventh", "Twelveth"
        ];

    let multiverse = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

// Still need to implement an 'And' on the second to last line of the multiverse.
    for i in 0..=11 {
        println!("\n[ {}. Verse ]\nOn the {} day of Christmas,\nMy true love sent to me",i+1, dayth[i]);
        for i in 0..=i {
            println!("{}", multiverse[i]);
            continue
        }
    }
}
