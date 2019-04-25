pub fn bloat<Coor, Key>(x: Coor) -> Key
where
    Coor: num::traits::int::PrimInt,
    Key: num::traits::int::PrimInt
        + std::ops::BitOrAssign
        + std::ops::BitAndAssign
        + std::ops::ShlAssign<usize>,
{
    let coor_siz = std::mem::size_of::<Coor>();
    let key_siz = std::mem::size_of::<Key>();
    let siz_rat = key_siz / coor_siz;
    if siz_rat < 1 {
        panic!("The key cannot be smaller than the coordinate.");
    }
    
    let mut result = Key::from(x).unwrap();
    if siz_rat == 1 {
        return result;
    }
    
    let a_zeros_then_b_ones = |a: usize, b: usize| {
        let key_one = Key::from(1).unwrap();
        let onez = (key_one<< b) - key_one;
        let mut n: Key = onez;
        let limit = (key_siz * 8 / (a + b)) - 1;
        for _ in 0..limit {
            n <<= a + b;
            n |= onez;
        }
        n
    };

    let get_mask = |x: usize| -> Key {
        if (siz_rat.is_power_of_two()) && ((x as u32) % (siz_rat.trailing_zeros()) != 0) {
            Key::max_value()
        } else {
            a_zeros_then_b_ones((1 << x) * (siz_rat - 1), 1 << x)
        }
    };

    let shift_bitor_mask = |x: &mut Key, y| {
        let shift_amt = (siz_rat - 1) << y;
        *x |= *x << shift_amt;
        (*x) &= get_mask(y)
    };

    let op_amt = coor_siz.next_power_of_two().trailing_zeros() + 3;
    (0..op_amt)
        .rev()
        .for_each(|q| shift_bitor_mask(&mut result, q as usize));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_BITS: u8 = 28;

    fn fmt_bin<T>(x: T) -> std::string::String
    where
        T: std::fmt::Binary,
    {
        let size = std::mem::size_of::<T>();
        if size == 16 {
            format!("{:#0130b}", x)
        } else if size == 8 {
            format!("{:#066b}", x)
        } else if size == 4 {
            format!("{:#034b}", x)
        } else if size == 2 {
            format!("{:#018b}", x)
        } else if size == 1 {
            format!("{:#010b}", x)
        } else {
            unimplemented!()
        }
    }

    fn bloat_slow<Coor, Key>(x: Coor) -> Key
    where
        Coor: num::traits::int::PrimInt + std::fmt::Binary,
        Key: num::traits::int::PrimInt + std::ops::BitOrAssign + std::ops::BitAndAssign,
        <Key as num_traits::Num>::FromStrRadixErr: std::fmt::Debug,
    {
        let coor_siz = std::mem::size_of::<Coor>();
        let coor_bits = coor_siz*8;
        let key_siz = std::mem::size_of::<Key>();
        let siz_rat = key_siz / coor_siz;
        if siz_rat == 1 {
            return Key::from(x).unwrap();
        }
        let key_zero = Key::from(0u8).unwrap();
        let coor_zero = Coor::from(0u8).unwrap();
        let mut result = key_zero;
        let get_mask_key = |b| (Key::from(1).unwrap())<<(b*siz_rat);
        let get_mask_coor = |b| (Coor::from(1).unwrap())<<b;
        let tst_bit = |a: Coor, b| (a & get_mask_coor(b)) != coor_zero;
        let set_bit = |a: &mut Key, b| (*a) |= get_mask_key(b);
        for bit_examined in 0..coor_bits {
            if tst_bit(x, bit_examined) {set_bit(&mut result, bit_examined)}
        }
        result
    }

    fn test_all_possible_values<Coor, Key>()
    where
        Coor: num::traits::int::PrimInt + std::fmt::Binary,
        Key: num::traits::int::PrimInt
            + std::ops::BitOrAssign
            + std::ops::BitAndAssign
            + std::fmt::Binary
            + std::ops::ShlAssign<usize>,
        <Key as num_traits::Num>::FromStrRadixErr: std::fmt::Debug,
        u128: std::convert::From<Coor>,
    {
        let fn_1 = bloat_slow::<Coor, Key>;
        let fn_2 = bloat::<Coor, Key>;
        let limit = std::cmp::min(u128::from(Coor::max_value()), 1u128 << MAX_BITS);
        let limit = limit as usize;
        for x in 0..=limit {
            let x_ = Coor::from(x).expect("Coor and usize incompatible.");
            if x%(1<<24) == 0 && x!=0 {dbg!(x>>24);}
            let fn1 = fn_1(x_);
            let fn2 = fn_2(x_);
            if fn1 != fn2 {
                panic!(
                    "x = {} \nfn_1 = {}, \nfn_2 = {}",
                    x,
                    fmt_bin(fn1),
                    fmt_bin(fn2)
                )
            }
        }
    }

    #[test]    fn test_u8_u16    () {        test_all_possible_values::<u8, u16>    ();    }
    #[test]    fn test_u8_u32    () {        test_all_possible_values::<u8, u32>    ();    }
    #[test]    fn test_u8_u64    () {        test_all_possible_values::<u8, u64>    ();    }
    #[test]    fn test_u8_u128  () {        test_all_possible_values::<u8, u128>  ();    }
    #[test]    fn test_u16_u32  () {        test_all_possible_values::<u16, u32>  ();    }
    #[test]    fn test_u16_u64  () {        test_all_possible_values::<u16, u64>  ();    }
    #[test]    fn test_u16_u128() {        test_all_possible_values::<u16, u128>();    }
    #[test]    fn test_u32_u64  () {        test_all_possible_values::<u32, u64>  ();    }
    #[test]    fn test_u32_u128() {        test_all_possible_values::<u32, u128>();    }
    #[test]    fn test_u64_u128() {        test_all_possible_values::<u64, u128>();    }
}
