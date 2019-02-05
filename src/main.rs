#[macro_use]
extern crate lazy_static;

mod gray;
use gray::{ printBin,toGray,fromGray,from_gray_bu,to_gray_bu };

use std::str::from_utf8;
use std::cmp::Ordering;
mod canvas;
mod individual;
use num::BigUint;
use num_traits::cast::FromPrimitive;
use individual::{ Individual,max_len,draw};
use canvas::{ Canvas,Vector4};
use rand::random;
use std::thread::sleep_ms;

const INDIVIDUAL_NUM:u32 = 30;
const ITER_NUM:usize = 500;
const MUTATION:f32 = 0.8;
type Population = Vec<Individual>;

fn main() {
    let mut population = init_population();
    let mut i = 0;
    let ml = max_len();
    println!("Max Len = {}",ml);

    loop{
        //if i >= ITER_NUM{ break;}
        let (index,i1) = selection(&mut population);
        let i2 = if index + 1 >= INDIVIDUAL_NUM as usize{
            population[ index - 1]
        }else { population[ index + 1] };
        //dbg!(i1);
        //dbg!(i2);
        //if dbg!( population[0].score() ) > 15.0 {break; }
        if population[0].score() > ml - 0.5{break; }
        population[INDIVIDUAL_NUM as usize - 1] =  cross_over(i1,i2);
        if random::<f32>() <= MUTATION { mutation(& mut population);}
        i += 1;
    }

    printBin(fromGray(population[0].gene));
    println!("{}",population[0].score());
    sleep_ms(1000);
    draw(population[0]);
}

fn init_population() -> Population
{
    let mut res = vec!();
    for _ in 0..INDIVIDUAL_NUM {
        //let t:Individual = random::<u32>().into();
        let t = Individual::rand();
        println!("{}",t.score());
        res.push(t);//Individual::rand());
    }
    res
}

fn selection(population: &mut Population) -> (usize,Individual)
{
    population.sort_by(| a,b | { 
        let s1 = a.score();
        let s2 = b.score();
        if s1 < s2 { Ordering::Greater  }else  {
            Ordering::Less 
        } 
    });
    //population.iter().for_each( |it| println!("{}",it.score()));
    let mut m:f32 = 0.0;
    population.iter().for_each( |it| m += it.score() );

    if m > 0.0 {
        let fs:Vec<f32> = population.iter().map(|it| it.score()/m  ).collect();
        let r = random::<f32>();
        for i in 0..fs.len(){
            if fs[i] >= r{
                return (i,population[i]);
            }
        }
    }
    let index = 0;
    (index,population[ 0])//random::<usize>() % INDIVIDUAL_NUM as usize ]
}

fn cross_over(i1: Individual,i2 : Individual ) -> Individual
{ 
    if i1.gene != i2.gene { 
        let mut res = 0u32;
        let r = random::<u32>() % 31 + 1 ; // 1 ~ 31
        let mask1 = get_mask(r) << (32 - r);
        let mask2 = get_mask(32 - r);
        res = mask1 & i1.gene;
        res |= (mask2 & i2.gene);
        //dbg!(r);
        //printBin(i1.gene);
        //printBin(i2.gene);
        //printBin(res);
        //return dbg!( res.into() );
        return res.into();
    }
    i1
}

fn mutation(population: &mut Population) 
{
    let index = random::<usize>() % INDIVIDUAL_NUM as usize;
    let mut ind = population[index ];
    for _ in 0..(random::<u32>() % 25 + 1){
        let r = random::<u32>() % 32;
        let k = 1u32 << r;
        //printBin(ind.gene);
        if ((ind.gene & k) >> r) == 0
        {
            ind.gene |= k;
        }else {
            ind.gene &= !k;
        }
        //dbg!(r);
        //printBin(ind.gene);
        population[index] = ind;
    }
}

fn get_mask(s:u32) -> u32
{
    let mut res = 0;
    let k = 1u32;
    for _ in 0..s{
        res |= k;
        res <<= 1;
    }
    res >>= 1;
    res
}

