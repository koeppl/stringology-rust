use num::cast::AsPrimitive;
use crate::io;
extern crate cdivsufsort;
extern crate log;
use log::debug;

pub fn bwt_from_text_by_sa(text: &Vec<u8>) -> Vec<u8> {
    let n = text.len();
    let mut sa = vec![0; n];
    assert!(!text[..text.len()-1].into_iter().any(|&x| x == 0));
    cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
    let mut bwt = vec![text[0]; n];
    // let mut rsa = vec![0; n];
    for i in 0..n {
        bwt[i] = text[(n + (sa[i] as usize)-1)  % n];
        // rsa[i] = (n + (sa[i] as usize)-1)  % n;
    }
    debug!("text: {:?}", text);
    debug!("bwt: {:?}", bwt);
    debug!("sa: {:?}", sa);
    // println!("rsa: {:?}", rsa);
    bwt
}


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

/// Duval's algorithm
/// returns a list of ending positions of the computed Lyndon factors.
/// Duval, Jean-Pierre (1983), "Factorizing words over an ordered alphabet", Journal of Algorithms,
/// 4 (4): 363–381, doi:10.1016/0196-6774(83)90017-2.
pub fn duval<C : Ord>(text: &[C]) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let n = text.len();
    while k < n {
        let mut i = k;
        let mut j = k + 1;
        while j != n && text[i] <= text[j] {
            if text[i] < text[j] {
                i = k;
            }
            if text[i] == text[j] {
                i += 1;
            }
            j += 1;
        }
        loop {
            assert_lt!(i,j);
            k += j-i;
            ending_positions.push(k-1 as usize);
            if k >= i { break }
        }
    }
    return ending_positions;
}

/// Lyndon factorization via the inverse suffix array
pub fn isa_lyndon_factorization(isa : &[i32]) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let mut current_val = isa[k];
    let n = isa.len();
    k += 1;
    while k < n {
        if isa[k] < current_val {
            ending_positions.push(k-1 as usize);
            current_val = isa[k];
        }
        k += 1;
    }
    ending_positions.push(n-1);
    return ending_positions;
}

pub struct RandomStringFactory {
    m_range : std::ops::Range<usize>,
    m_log_alphabet_size : u8,
}

impl RandomStringFactory {
    #[allow(dead_code)]
    pub fn new(r : std::ops::Range<usize>, log_alphabet_size : u8) -> RandomStringFactory {
        RandomStringFactory { m_range : r, m_log_alphabet_size : log_alphabet_size}
    }
}


impl Iterator for RandomStringFactory {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> { //TODO: not long enough! -> need random strings!
        if self.m_range.start*2 >= self.m_range.end {
            None
        } else if self.m_range.start >= self.m_range.end {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut text = vec![0u8;128];
            for i in 1..text.len() {
                text[i] = rng.gen_range(1..2);
            }
            Some(text)
        } else {
            self.m_range.start += 1;
            let iter_round = self.m_range.start;
            debug_assert_lt!(std::mem::size_of_val(&iter_round)*8, 200);
            let most_significant_bit = bit_size(iter_round);

            let alphabet_mask = std::usize::MAX >> ((std::mem::size_of_val(&std::usize::MAX)*8) as u8 - self.m_log_alphabet_size);
            let mut text = Vec::new();
            for i in 1..(most_significant_bit/self.m_log_alphabet_size) as usize {
                text.push((((iter_round >> (self.m_log_alphabet_size*i as u8)) & alphabet_mask) + 1) as u8);
            }
            text.push(0);
            Some(text)
        }
    }
}

/// counts the number of runs in an array `arr`
pub fn number_of_runs<R : std::io::Read>(reader : &mut R) -> usize {
    match io::read_char(reader) {
        Err(_) => return 0,
        Ok(first_char) => {
            let mut run_counter = 1; //@ counts the number of character runs
            let mut prev_char = first_char; //@ the current character of the chracter run
            loop {
                match io::read_char(reader) {
                    Err(_) => break,
                    Ok(next_char) => {
                        if next_char != prev_char {
                            prev_char = next_char;
                            run_counter += 1;
                        }
                    }
                }
            }
            run_counter
        }
    }
}
