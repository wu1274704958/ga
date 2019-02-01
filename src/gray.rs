use num::PrimInt;
use std::mem::{ size_of, transmute };
use std::ops::{ Shl,Shr};
use std::fmt::Debug;


pub fn printBin<T>(b: T)
    where T : PrimInt + Debug
{
    let len = size_of::<T>() * 8 - 1;
    let K  = T::one() << len;
    let mut i = len as i32;
    loop
        {
            if !(i >= 0) {break;}

            if K & (b << (len - i as usize)) == T::zero() {
                print!("0");
            }else { print!("1"); }
            if i % 4 == 0 {
                print!(" ");
            }
            i -= 1;
        }
    println!();
}

pub fn toGray<T>(b : T) -> T
    where T : PrimInt
{
    let K:T = T::one();
    let len = size_of::<T>() * 8 - 1;
    let mut j = len;
    let mut f = K & (b >> j);
    let mut res = f << j;

    loop{
        if !(j > 0) { break;}
        let nf = (K & (b >> ( j - 1 )));
        res = res | ((f ^ nf) << (j - 1));
        f = nf;
        j -= 1;
    }
    return res;
}

pub fn fromGray<T>(g : T) -> T
    where T : PrimInt
{
    let K:T = T::one();
    let len = size_of::<T>() * 8 - 1;
    let mut j = len;
    let mut res = (K & (g >> j)) << j;

    loop{
        if !(j > 0) { break;}
        let nf = (K & (res >> j));
        let gi = (K & (g >> (j - 1)));
        res = res | ((gi ^ nf) << (j - 1));
        j -= 1;
    }
    return res;
}