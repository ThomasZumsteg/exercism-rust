extern crate rand;
use rand::Rng;

pub fn private_key(_p: u64) -> u64 {
    rand::thread_rng().gen_range(2, _p)
}

pub fn public_key(_p: u64, _g: u64, _a: u64) -> u64 {
    _g.pow(_a as u32) % _p
}

pub fn secret(_p: u64, _b_pub: u64, _a: u64) -> u64 {
    public_key(_p, _b_pub, _a)
}
