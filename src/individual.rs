use std::convert::From;
use std::fmt::Debug;
use crate::gray::{ fromGray,toGray};
use rand::random;

const BX:i32 = 0;
const BY:i32 = 0;

const EX:i32 = 16;
const EY:i32 = 16;

const W:i32 = 16;
const H:i32 = 16;

const STONE_NUM:u32 = 2;

lazy_static! {
    static ref Stones:Vec<i32> = {
        init_stones()
    };
    static ref MaxLen:f32 = { 
        max_len()
    };
}
pub fn max_len() -> f32
{
    (((EX - BX).pow(2) + (EY - BY).pow(2))as f32).sqrt()
}

pub enum Behavior {Up, Down, Left, Right}

#[derive(Debug,Copy,Clone)]
pub struct Individual{
    pub gene : u32
}

impl From<u32> for Individual {
    fn from(w: u32) -> Self {
         Individual { gene : w }
    }
}

impl Individual{
    pub fn score(&self) -> f32
    {
        let k = 3u32;
        let b = fromGray(self.gene);
        let mut x = BX;
        let mut y = BY;
        
        for i in 0..16{
            let pre_x = x;
            let pre_y = y;

            let a = k & (b >> (i << 1));
            let beh = to_behavior(a);
            match beh{
                Behavior::Up => y -= 1,
                Behavior::Down => y += 1,
                Behavior::Left => x -= 1,
                Behavior::Right => x += 1
            }

            if has_stone(x,y)
            {
                x = pre_x;
                y = pre_y;
                break;
            }
        }

        let len = (((y - EY).abs().pow(2) + (x - EX).abs().pow(2))as f32).sqrt();
        (*MaxLen) - len
    }
    pub fn rand() -> Individual
    {
        let mut b:u32 = 0;
        for i in 0..16{
            let r = random::<u32>() % 4;
            b = b << (i << 1);
            b = b | r;
        }
        toGray(b).into()
    }
}

pub fn init_stones() -> Vec<i32>
{
    let mut stones = Vec::new();
    for i in 0..STONE_NUM{
        stones.push(random::<i32>() % W);
        stones.push(random::<i32>() % H);
    }
    stones
}

pub fn has_stone(x:i32,y:i32) -> bool
{
    let len = unsafe {Stones.len() };
    let mut i = 0usize;
    loop{
        if i >= len{ break;}
        if unsafe { Stones[i] == x && Stones[i + 1] == y }
        {  return true; }
        i += 2;
    }
    false
}

pub fn to_behavior(b:u32) -> Behavior
{
    match b {
        0 => Behavior::Up,
        1 => Behavior::Down,
        2 => Behavior::Left,
        3 => Behavior::Right,
        _ => {panic!("no behavior")}
    }
}
