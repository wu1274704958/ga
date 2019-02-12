use std::convert::From;
use std::fmt::{Debug,Result};
use crate::gray::{ fromGray,toGray};
use rand::random;
use crate::canvas::{ Canvas };
use std::thread::sleep;
use console::{style,Term};
use std::str::from_utf8;
use num::PrimInt;
use std::mem::size_of;
use core::time::Duration;
use std::iter::FromIterator;

const BX:i32 = 0;
const BY:i32 = 0;

const EX:i32 = 5;
const EY:i32 = 12;

const W:i32 = 16;
const H:i32 = 16;

const STONE_NUM:u32 = 7;

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

impl Debug for Behavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        let c = match self{
           Behavior::Up      =>'↑',
           Behavior::Down    =>'↓',
           Behavior::Left    =>'←',
           Behavior::Right   =>'→'
        };
        write!(f, "{}", c)                                  
    }                                                                          
} 

#[derive(Debug,Copy,Clone)]
pub struct Individual<T:PrimInt>{
    pub gene : T
}

impl<T:PrimInt> From<T> for Individual<T> {
    fn from(w: T) -> Self {
         Individual { gene : w }
    }
}

impl<T:PrimInt> Individual<T>{
    pub fn score(&self) -> f32
    {
        let k:T = T::from(3).unwrap();
        let b = fromGray(self.gene);
        let mut x = BX;
        let mut y = BY;
        
        for i in 0..size_of::<T>() * 4 {
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
            let len = (((y - EY).abs().pow(2) + (x - EX).abs().pow(2))as f32).sqrt();
            if len < 0.5{
               return (*MaxLen) - len + (size_of::<T>() * 4 - 1 - i) as f32;
            }
        }

        let len = (((y - EY).abs().pow(2) + (x - EX).abs().pow(2))as f32).sqrt();
        (*MaxLen) - len
    }
    pub fn rand() -> Individual<T>
    {
        let mut b:T = T::zero();
        for i in 0..size_of::<T>() * 4{
            let r = random::<u32>() % 4;
            b = b << (i << 1);
            b = b | T::from(r).unwrap();
        }
        toGray(b).into()
    }
}

pub fn init_stones() -> Vec<i32>
{
    let mut stones = Vec::new();
    let mut i = 0;
    loop
    {
        if i >= STONE_NUM { break; }
        let x = (random::<u32>() % W as u32) as i32;
        let y = (random::<u32>() % H as u32) as i32;
        if x != EX && y != EY && !has_stone_init(&stones,x,y) {
            stones.push(x);
            stones.push(y);
        }else { continue; }
        i += 1;
    }
    stones
}

fn has_stone_init(stones: &Vec<i32>,x :i32, y :i32) -> bool
{
    let len = stones.len();
    let mut i = 0usize;
    loop{
        if i >= len{ break;}
        if  stones[i] == x && stones[i + 1] == y
        {  return true; }
        i += 2;
    }
    false
}

pub fn has_stone(x:i32,y:i32) -> bool
{
    let len = Stones.len() ;
    let mut i = 0usize;
    loop{
        if i >= len{ break;}
        if  Stones[i] == x && Stones[i + 1] == y
        {  return true; }
        i += 2;
    }
    false
}

pub fn to_behavior<T :PrimInt>(b:T) -> Behavior
{
    match b.to_u32().unwrap() {
        0 => Behavior::Up,
        1 => Behavior::Down,
        2 => Behavior::Left,
        3 => Behavior::Right,
        _ => {panic!("no behavior")}
    }
}
#[allow(unused_must_use)]
pub fn draw<T:PrimInt>(ind : Individual<T>)
{
    let mut c = Canvas::new(W as u32,H as u32);
    c.setPixel2D(BX,BY,'始');
    c.setPixel2D(EX,EY,'終');

    let stdout = Term::stdout();
    let mut x = BX;
    let mut y = BY;
    let k = T::from(3u32).unwrap();
    let b = fromGray(ind.gene);
    let score = ind.score();
    
    let len = Stones.len() ;
    let mut j = 0usize;
    loop{
        if j >= len{ break;}
        c.setPixel2D(Stones[j], Stones[j + 1],'石');
        j += 2;
    }
        
    for i in 0..size_of::<T>() * 4{
        if i != 0 { stdout.move_cursor_up(H as usize + 1); }
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
            break;
        }
        if x == EX && y == EY
        {
            c.setPixel2D(x,y,'逹');
            let s =  String::from_iter( c.data.iter() );
            print!("{}",style(s).cyan().on_black().bold());
            let tips = format!("{}  {:?} {}",i,beh,score);
            println!("{}",style(tips).green().on_black().bold());
            break;
        }
        let dir_c =  if stdout.features().is_msys_tty() {  get_dir_char_msys(&beh) }  else { get_dir_char(&beh) };
        c.setPixel2D(x,y,dir_c);
        let s = String::from_iter( c.data.iter() );
        print!("{}",style(s).cyan().on_black().bold());
        let tips = format!("{}  {:?} {}",i,beh,score);
        println!("{}",style(tips).green().on_black().bold());
        sleep( Duration::from_millis(300));
    }
}

#[cfg(target_os = "windows")]
fn get_dir_char(beh :&Behavior) ->char
{
    match beh{
        Behavior::Up      => '↑',
        Behavior::Down    => '↓',
        Behavior::Left    => '←',
        Behavior::Right   => '→'
    }
}

fn get_dir_char_msys(beh :&Behavior) ->char
{
    match beh{
        Behavior::Up      => '上',
        Behavior::Down    => '下',
        Behavior::Left    => '左',
        Behavior::Right   => '右'
    }
}

#[cfg(target_os = "linux")]
fn get_dir_char(beh :&Behavior) ->char
{
    match beh{
        Behavior::Up      => '㊤',
        Behavior::Down    => '㊦',
        Behavior::Left    => '㊧',
        Behavior::Right   => '㊨'
    }
}