extern crate itertools;
extern crate rand;
extern crate num_traits;
extern crate rayon;



mod bloatables;

use itertools::Itertools;
//use bloatables;
#[allow(unused_imports)]
use rayon::prelude::*;



pub fn get_morton_key<NormCoor, Key, NormSamp>(norm_samp: NormSamp) -> Key
where NormCoor: num_traits::int::PrimInt,
      Key: num_traits::int::PrimInt + std::ops::BitOrAssign + std::ops::BitAndAssign + std::ops::ShlAssign<usize>,
      NormSamp: IntoIterator<Item = NormCoor>,
      <NormSamp as IntoIterator>::IntoIter: ExactSizeIterator
{
    let eater = norm_samp.into_iter();
    
    let bloat_fn = |x: NormCoor| bloatables::bloat::<NormCoor, Key>(x);
    eater.map(bloat_fn)
            .fold(Key::from(0).unwrap(), |acc, x| (acc << 1) | x)
}


pub fn zbox_merge<H, Smp, Key, Set>(set: Set, get_key_from_sample: H) -> Vec<u8>
where
    H: Fn(Smp) -> Key + std::marker::Sync + std::marker::Send,
    //H: std::ops::Fn<(<Set as rayon::iter::IntoParallelIterator>::Item,)>,
    Key: num_traits::int::PrimInt,
    Set: IntoIterator<Item = Smp>,
    //Set: rayon::iter::IntoParallelIterator
{
    //set.into_par_iter()
    set.into_iter()
        .map(get_key_from_sample)
        .sorted()
        .tuple_windows()
        .map(|(a, b)| a ^ b)
        .map(|x| x.leading_zeros() as u8)
        .collect()
}

pub fn get_inclination(input: &[f64]) -> f64
{
    let length = input.iter().count() as f64;
    let avy: f64 = input.iter().sum::<f64>()/length;
    let avx: f64 = length*(length-1.0)/(2.0 * length);
    let num_inc = |(i, x): (usize, f64)| -> f64 {(x-avy)*(i as f64 - avx)};
    let denom_inc = |(i, _): (usize, f64)| -> f64 {(i as f64 - avx)*(i as f64 - avx)};
    let num: f64 = input.iter().enumerate().map(|(a, b): (usize, &f64)| num_inc((a, *b))).sum();
    let denom: f64 = input.iter().enumerate().map(|(a, b): (usize, &f64)| denom_inc((a, *b))).sum();
    num/denom
}

pub fn get_results_from_clzs(input: Vec<u8>, key_bit_amt: u8) -> (Vec<u32>, Vec<u64>) {
    let mut s: Vec<u32> = vec![0; key_bit_amt as usize];
    let mut prevs: Vec<usize> = vec![0; key_bit_amt as usize];
    let mut squares: Vec<u64> = vec![0; key_bit_amt as usize];
    for (i, x) in input.iter().chain([0].iter()).enumerate() {
        for b_i in (*x as usize)..(key_bit_amt as usize) {
            //let b_i = bit_iter as usize;
            s[b_i] += 1;
            squares[b_i] += (i - prevs[b_i]) as u64 * (i - prevs[b_i]) as u64;
            prevs[b_i] = i;
        }
    }
    (s, squares)
}

pub fn finalise_results(s: Vec<u32>, squares: Vec<u64>, sample_size: u32, coor_bit_amt: u8, key_bit_amt: u8) -> (f64, Vec<f64>, Vec<f64>) {
    let step = (key_bit_amt/coor_bit_amt) as usize;
    let result_2 = s.iter().skip(step-1).step_by(step).map(|&x| f64::from(x).log2()).collect_vec();
    let result_3 = squares.into_iter().zip(s.into_iter()).skip(step-1).step_by(step).map(|(a, b)| (a as f64)*(b as f64)/(sample_size as f64 * sample_size as f64) - 1.0).collect_vec();
    let cap = (sample_size as f64).log2();
    let result_1_lim = result_2.iter().position(|x| *x>(0.9)*cap).unwrap_or(coor_bit_amt as usize);
    let result_1 = get_inclination(&result_2[0..result_1_lim]);
    (result_1, result_2, result_3)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
