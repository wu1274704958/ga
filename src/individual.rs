use std::convert::From;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Individual{
    gene : u32
}

impl From<u32> for Individual {
    fn from(w: u32) -> Self {
         Individual { gene : w }
    }
}

impl Individual{

}
