fn main() {
    let days = [
        "On the first day of Christmas\n",
        "On the second day of Christmas\n",
        "On the third day of Christmas\n",
        "On the fourth day of Christmas\n",
        "On the fifth day of Christmas\n",
        "On the sixth day of Christmas\n",
        "On the seventh day of Christmas\n",
        "On the eight day of Christmas\n",
        "On the ninth day of Christmas\n",
        "On the tenth day of Christmas\n",
        "On the eleventh day of Christmas\n",
        "On the twelve day of Christmas\n",
    ];
    let friends = "My good friends brought to me\n";
    let gifts = [
        "And a song for the Christmas tree\n",
        "(Two candy canes)\n",
        "(Three boughs of holly)\n",
        "(Four colored lights)\n",
        "(A shining star)\n",
        "(Little silver bells)\n",
        "(Candles a glowing)\n",
        "(Gold and silver tinsel)\n",
        "(A guardian angel)\n",
        "(Some mistletoe)\n",
        "(Gifts for one and all)\n",
        "(All their good wishes)\n",
    ];

    let mut lyrics = String::from("");
    lyrics = lyrics + days[0] + friends + "A song and a Christmas tree\n";
    for i in 1..days.len() {
        lyrics = lyrics + days[i] + friends;
        for j in (0..i+1).rev() {
            lyrics += gifts[j];
        }
    }
    println!("{}", lyrics);
}
