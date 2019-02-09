#[macro_use]
extern crate lazy_static;

mod gray;
use gray::{ printBin,fromGray };

use std::cmp::Ordering;
#[allow(dead_code)]
#[allow(non_snake_case)]
mod canvas;
mod individual;
use individual::{ Individual,max_len,draw};
use rand::random;
use std::thread::sleep;
use core::time::Duration;

const INDIVIDUAL_NUM:u32 = 30;
#[allow(dead_code)]
const ITER_NUM:usize = 500;
const MUTATION:f32 = 0.8;
type Population = Vec<Individual<u64>>;

fn main() {
    let mut population = init_population();
    let mut _i = 0;
    let ml = max_len() + 10.0;
    println!("Max Len = {}",ml);

    loop{
        //if _i >= ITER_NUM{ break;}
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
        _i += 1;
    }

    printBin(fromGray(population[0].gene));
    println!("{}",population[0].score());
    sleep(Duration::from_millis(1000));
    draw(population[0]);
}

fn init_population() -> Population
{
    let mut res = vec!();
    for _ in 0..INDIVIDUAL_NUM {
        //let t:Individual = random::<u32>().into();
        let t = Individual::<u64>::rand();
        println!("{}",t.score());
        res.push(t);//Individual::rand());
    }
    res
}

fn selection(population: &mut Population) -> (usize,Individual<u64>)
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
#[allow(unused_assignments)]
fn cross_over(i1: Individual<u64>,i2 : Individual<u64> ) -> Individual<u64>
{ 
    if i1.gene != i2.gene { 
        let mut res = 0u64;
        let r = random::<u32>() % 63 + 1 ; // 1 ~ 63
        let mask1 = get_mask(r) << (64 - r);
        let mask2 = get_mask(64 - r);
        res = mask1 & i1.gene;
        res |= mask2 & i2.gene;
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
        let r = random::<u32>() % 64;
        let k = 1u64 << r;
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

fn get_mask(s:u32) -> u64
{
    let mut res = 0;
    let k = 1u64;
    for _ in 0..s{
        res |= k;
        res <<= 1;
    }
    res >>= 1;
    res
}

