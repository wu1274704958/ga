mod gray;
use num::BigUint;
use gray::{ printBin,toGray,fromGray };

use std::str::from_utf8;
mod canvas;
mod individual;
use individual::Individual;
use canvas::{ Canvas,Vector4};
const individual_num :u32 = 6; 

type Population = Vec<Individual>;

fn main() {
    let a = toGray(1028i64);
    printBin(1028i64);
    printBin(a);
    printBin(fromGray(a));

    let population = init_population();
    population.iter().for_each(|it|{
        println!("{:?}",it);
    });

    let a:Individual = 0u32.into();
    let mut can = Canvas::new(40,80);
    can.drawLine(&Vector4{ x:0.0,y:0.0,z:0.0,w:0.0}, &Vector4 { x:20.0,y:20.0,z:20.0,w:0.0});
    let s = from_utf8(can.data.as_ref()).unwrap();
    println!("{}",s);
    
}

fn init_population() -> Population
{
    let mut res = vec!();
    for _ in 0..individual_num{
        res.push(0u32.into());
    }
    res
}
