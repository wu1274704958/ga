mod gray;
use gray::{ printBin,toGray,fromGray,to_gray_bu,from_gray_bu};

use std::str::from_utf8;
mod canvas;
mod individual;
use num::BigUint;
use num_traits::cast::FromPrimitive;
use individual::Individual;
use canvas::{ Canvas,Vector4};
const individual_num :u32 = 6; 

type Population = Vec<Individual>;

fn main() {
    let a = toGray(1028i64);
    printBin(1028i64);
    printBin(a);
    printBin(fromGray(a));
    println!("{:?}",a);

    let population = init_population();
    population.iter().for_each(|it|{
        println!("{:?}",it);
    });

    let a:Individual = 0u32.into();

    let bu = BigUint::from_u64(1028).unwrap();
    println!("{:?}",bu);
    let bu_res = to_gray_bu(bu);
    println!("{:?}",bu_res);
    println!("{:?}",from_gray_bu(bu_res));
    //let mut can = Canvas::new(40,80);
    //can.drawLine(&Vector4{ x:0.0,y:0.0,z:0.0,w:0.0}, &Vector4 { x:20.0,y:20.0,z:20.0,w:0.0});
    //let s = from_utf8(can.data.as_ref()).unwrap();
    //println!("{}",s);
    
}

fn init_population() -> Population
{
    let mut res = vec!();
    for _ in 0..individual_num{
        res.push(0u32.into());
    }
    res
}
