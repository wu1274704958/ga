mod gray;

use gray::{ printBin,toGray,fromGray };

fn main() {
    let a = toGray(1028i64);
    printBin(1028i64);
    printBin(a);
    printBin(fromGray(a));
}

