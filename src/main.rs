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
use individual::Individual;
use canvas::{ Canvas,Vector4};
use rand::random;
const INDIVIDUAL_NUM:u32 = 30;
const ITER_NUM:usize = 5000;
const MUTATION:f32 = 0.1;
type Population = Vec<Individual>;

fn main() {
    let mut population = init_population();
    let mut i = 0;
    loop{
        if i >= ITER_NUM{ break;}
        let i1 = selection(&mut population);
        let i2 = selection(&mut population);
        cross_over(i1,i2);
        if random::<f32>() <= MUTATION { mutation(&population);}
        i += 1;
    }

    printBin(fromGray(population[0].gene));
    println!("{}",population[0].score());
}

fn init_population() -> Population
{
    let mut res = vec!();
    for _ in 0..INDIVIDUAL_NUM {
        res.push(random::<u32>().into());//Individual::rand());
    }
    res
}

fn selection(population: &mut Population) -> Individual
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
                return population[i];
            }
        }
    }
    population[ random::<usize>() % INDIVIDUAL_NUM as usize ]
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
        return res.into();
    }
    i1
}

fn mutation(population: &Population) -> Individual
{
    let mut ind = population[ random::<usize>() % INDIVIDUAL_NUM as usize ];
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
    ind
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

