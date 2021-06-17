use num::cast::AsPrimitive;

/// compute the location of the most significant bit 
pub fn bit_size(i : usize) -> u8 {
    ((std::mem::size_of_val(&i)*8) as u8) - i.leading_zeros() as u8
}


pub fn compute_phi<T : AsPrimitive<usize> + num::cast::FromPrimitive>(sa : &[T]) -> Vec<T> {
    let mut phi = vec![T::from_usize(0).unwrap(); sa.len()]; 
    for i in 1..sa.len()  {
        phi[sa[i].as_() as usize] = sa[i-1];
    }
    phi[sa[0].as_() as usize] = sa[sa.len()-1];
    phi
}

pub fn inverse_permutation<T : AsPrimitive<usize> + num::cast::FromPrimitive>(arr : &[T]) -> Vec<T> {
    let mut inv = vec![T::from_usize(0).unwrap(); arr.len()]; 
    for i in 0..arr.len()  {
        inv[arr[i].as_() as usize] = T::from_usize(i).unwrap();
    }
    inv
}

pub fn compute_plcp(text: &[u8], phi: &[i32]) -> Vec<u32> {
    debug_assert_eq!(text.len(), phi.len());
    let mut plcp = vec![0; text.len()]; 
    let mut length : usize = 0;
    for position_b in 0..text.len() {
        let position_a = phi[position_b] as usize;
        //@ the first conditions do not need to be checked if we can ensure that text ends with a
        //@ unique delimiter such as 0-byte
        while position_a+length < text.len() && position_b+length < text.len() && text[(position_a+length) as usize] == text[(position_b+length) as usize] {
            length+=1;
        }
        plcp[position_b] = length as u32;
        if length > 0 {
            length -= 1;
        }
    }
    plcp
}

pub fn compute_lcp<T : AsPrimitive<usize> + num::cast::FromPrimitive>(plcp : &[u32], sa : &[T]) -> Vec<u32> {
    debug_assert_eq!(plcp.len(), sa.len());
    let mut lcp = vec![0; plcp.len()]; 
    for i in 0..lcp.len() {
        lcp[i] = plcp[sa[i].as_() as usize]
    }
    lcp
}

pub const INVALID_VALUE :u32 = std::u32::MAX;

pub fn compute_psv<T : Ord>(arr : &[T]) -> Vec<u32> {
    let mut psv = vec![0; arr.len()];
    psv[0] = INVALID_VALUE;
    for i in 1..arr.len() {
        let mut p = i-1;
        psv[i] = INVALID_VALUE;
        while p != INVALID_VALUE as usize { 
            if arr[p] < arr[i] {
                psv[i] = p as u32;
                break;
            } 
            p = psv[p] as usize;
        }
    }
    psv
}

pub fn compute_nsv<T : Ord>(arr : &[T]) -> Vec<u32> {
    let mut nsv = vec![0; arr.len()];
    nsv[arr.len()-1] = INVALID_VALUE;
    for i in (0..arr.len()-1).rev() {
        let mut p = i+1;
        nsv[i] = INVALID_VALUE;
        while p != INVALID_VALUE as usize { 
            if arr[p] < arr[i] {
                nsv[i] = p as u32;
                break;
            } 
            p = nsv[p] as usize;
        }
    }
    nsv
}
