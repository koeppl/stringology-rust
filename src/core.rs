use num::cast::AsPrimitive;
use crate::io;
extern crate cdivsufsort;
extern crate log;
use log::debug;
use more_asserts::debug_assert_lt;
use more_asserts::assert_gt;
use more_asserts::assert_lt;

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

/// computes the rightmost column of the BWT matrix
/// note that this is a O(n^2 lg n) algorithm!
pub fn bwt_by_matrix_naive<T : std::cmp::Ord + Copy>(text: &[T]) -> Vec<T> {
    let mut indices = Vec::with_capacity(text.len());
    for i in 0..text.len() {
        indices.push(i);
    }
    indices.sort_by(|a, b| -> std::cmp::Ordering { 
        for i in 0..text.len() {
            let cmp = text[(a+i) % text.len()].cmp(&text[(b+i) % text.len()]);
            if cmp == std::cmp::Ordering::Equal {
                continue;
            }
            return cmp;
        }
        return std::cmp::Ordering::Equal;
    });
    let mut bwt = Vec::with_capacity(text.len());
    for i in 0..text.len() {
        bwt.push(text[(indices[i]+text.len()-1) % text.len()]);
    }
    bwt
}

/// computes the BWT based on the matrix, i.e., the sorting of the cyclic conjugates of the 
/// input text by first finding its Lyndon conjugate, appending a 0 byte, 
/// then computing the BWT of this conjugate
/// via the suffix array, and removing the 0 byte at the end.
pub fn bwt_by_matrix(text: &[u8]) -> Vec<u8> {
   let n = text.len();
    assert_gt!(n, 0);

    let conjugate_start = lyndon_conjugate(&text);
    let mut newtext = Vec::new();
    newtext.reserve(n);
    for i in conjugate_start..n {
        newtext.push(text[i]);
    }
    for i in 0..conjugate_start {
        newtext.push(text[i]);
    }
    newtext.push(0u8);
    let mut bwt = bwt_from_text_by_sa(&newtext);
    bwt.remove(bwt.iter().position(|&x| x == 0).unwrap());
    bwt
}


/// the conjugate of the text that is Lyndon
pub fn lyndon_conjugate<C : Ord + Copy + Clone>(text: &[C]) -> usize {
   let n = text.len();
   let mut doubletext = Vec::new();
   doubletext.extend_from_slice(text);
   doubletext.extend_from_slice(text); //@ TODO: instead of doubling the text, we could write a wrapper around the text

   let mut factors = duval(&doubletext);
   factors.push(2*n);
   if factors[0]+1 == n { return 0; } //@ if the first factor ends at position n, we are done
   for x in 0..factors.len()-1 {
      if factors[x+1]+1 >= n {
         return factors[x]+1;
      }
   }
   assert!(false); //should never happen
   return 0;
}

/// a string is primitive if it is not the x-times concatenation of a string, for x being an
/// integer >= 2
pub fn is_primitive<C : Eq>(text: &[C]) -> bool {
    let border = border_array(text);
    let period = smallest_period(&border);
    if period == text.len() {
       return true;
    }
    (text.len() % smallest_period(&border)) != 0
}

/// the smallest period of a string is given by the  last entry of its border array
pub fn smallest_period(border_array : &[usize]) -> usize {
    let n = border_array.len()-1;
    n - border_array[n]
}

/// computes the bijective Burrows-Wheeler transform naively
pub fn bbwt_naive<C: Ord + Clone + Copy>(text: &[C]) -> Vec<C> {
    let n = text.len();
    let factors = duval(&text);
    // factors.push(n-1);
	// println!("duval factors {:?}", factors);

    struct Conjugate {
        index : usize, //@ conjugate number
        lyndon_start : usize,
        lyndon_end : usize
    }
    let mut conjugates = Vec::new();

    // if factors[0] > 0 {
    for j in 0..factors[0]+1 {
        conjugates.push(Conjugate { 
            index : j, 
            lyndon_start : 0, 
            lyndon_end : factors[0] });
    }
    // }

    for factor in 0..factors.len()-1 {
        for j in factors[factor]+1..factors[factor+1]+1 {
            conjugates.push(Conjugate { 
                index : j-factors[factor]-1, 
                lyndon_start : factors[factor]+1, 
                lyndon_end : factors[factor+1] });
        }
    }
    assert_eq!(conjugates.len(), n);
    conjugates.sort_by(|a, b| {
        let mut len = 0;
        let alen =1+ a.lyndon_end-a.lyndon_start;
        let blen =1+ b.lyndon_end-b.lyndon_start;
        while len < alen*blen {
            let char_a = text[a.lyndon_start + ((a.index+len) % alen )];
            let char_b = text[b.lyndon_start + ((b.index+len) % blen )];
            if char_a == char_b { 
                len += 1;
                continue; 
            }
            if char_a < char_b { return std::cmp::Ordering::Less; } else { return std::cmp::Ordering::Greater; }
        };
        return std::cmp::Ordering::Equal;
    });
    let mut bbwt = vec![text[0]; n];
    for i in 0..n {
        let position = if conjugates[i].index == 0 { 
            conjugates[i].lyndon_end
        } else { 
            conjugates[i].lyndon_start + conjugates[i].index - 1
        };
        bbwt[i] = text[position];
    }
    bbwt
}

/**
 * C : character trait. Must be of type `Ord`
 */
pub fn suffixarray_naive<C : Ord>(text: &[C]) -> Vec<usize> {
    let n = text.len();
    let mut sa = vec![0 as usize; n];
    for i in 0..n {
        sa[i] = i;
    }
    sa.sort_by(|a, b| {
        let asuffix = &text[*a..];
        let bsuffix = &text[*b..];
        asuffix.cmp(bsuffix)
    });
    sa
}

/**
 * bwt is a permutation of `text` based on `sa`
 */
pub fn bwt_from_sa<C : Clone + Copy>(text: &[C], sa: &Vec<usize>) -> Vec<C> {
    let n = text.len();
    let mut bwt = vec![text[0]; n];
    for i in 0..n {
        bwt[i] = text[(n + (sa[i] as usize)-1)  % n];
    }
    bwt
}

/**
 * the border array B of text[0..n) of length n+1
 * It uses a dummy 0 value at the beginning (often -1 in the literature)
 * to speed up computation by avoiding an additional if instruction.
 * B[i] stores the longest prefix of text that is a suffix of text[1..i).
 * Hence, B[0] and B[1] are dummy values having to meaning.
 *
 */
pub fn border_array<C : Eq>(text: &[C]) -> Vec<usize> {
    let n = text.len();
    let mut border = Vec::new();
    border.reserve(n+1);
    border.push(0);
    for i in 0..n {
        let mut length = border[i];
        while length > 0 && text[length-1] != text[i] {
            length = border[length-1];
        }
        border.push(length+1);
    }
    for i in 1..border.len() {
        border[i] -= 1;
    }
    border
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

pub struct RandomStringGenerator {
    m_range : std::ops::Range<usize>,
    m_log_alphabet_size : u8,
}

impl RandomStringGenerator {
    #[allow(dead_code)]
    pub fn new(r : std::ops::Range<usize>, log_alphabet_size : u8) -> RandomStringGenerator {
        RandomStringGenerator { m_range : r, m_log_alphabet_size : log_alphabet_size}
    }
}


impl Iterator for RandomStringGenerator {
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

pub struct LyndonWordGenerator {
   m_length : usize,
   m_alphabet_size : usize,
   m_stack : Vec<u8> 
}

impl LyndonWordGenerator {
    #[allow(dead_code)]
    pub fn new(length : usize, alphabet_size : usize) -> LyndonWordGenerator {
        LyndonWordGenerator { m_length : length, m_alphabet_size : alphabet_size, m_stack : Vec::new() }
    }
}

impl Iterator for LyndonWordGenerator {
    type Item = Vec<u8>;

    /// Copied from David Eppstein @ https://www.ics.uci.edu/~eppstein/PADS/Lyndon.py
    /// Generate nonempty Lyndon words of length <= n over an s-symbol alphabet.
    /// The words are generated in lexicographic order, using an algorithm from
    /// J.-P. Duval, Theor. Comput. Sci. 1988, doi:10.1016/0304-3975(88)90113-2.
    /// As shown by Berstel and Pocchiola, it takes constant average time
    /// per generated word.
    fn next(&mut self) -> Option<Vec<u8>> { 
       if self.m_length == 0 { return None; }
       let stack = &mut self.m_stack;
       if stack.is_empty() {
          stack.push(0); //@ set up for first increment
          return Some(stack.clone());
       }

       while !stack.is_empty() {
          let current_length = stack.len();
          while stack.len() < self.m_length {              //@ repeat word to fill exactly n syms
             stack.push(stack[stack.len()-current_length]);
          }

          //@ delete trailing z's
          while !stack.is_empty() && *stack.last().unwrap() as usize == self.m_alphabet_size - 1 { 
             stack.pop();
          }
          if stack.is_empty() { 
             self.m_length = 0;   //@ prevent from re-initialization by making this iterator invalid
             return None; 
          }
          else {
             *stack.last_mut().unwrap() += 1;                      //@ increment the last non-z symbol
             return Some(stack.clone());
          }
       }
       self.m_length = 0;
       return None;
    }
}

/// Iterates over all binary strings of a given length in lexicographic order
pub struct BinaryStringGenerator {
   m_length : u8,
   m_rank : u64,
}

impl BinaryStringGenerator {
    #[allow(dead_code)]
    pub fn new(length : u8) -> BinaryStringGenerator {
        BinaryStringGenerator { m_length : length, m_rank : 0 }
    }
}

impl Iterator for BinaryStringGenerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> { 
       if self.m_rank == u64::MAX { return None; }
       let mut text = Vec::new();
       text.reserve(self.m_length as usize);
       for i in 0..self.m_length {
          let bit = self.m_rank & (1<<i);
          text.push( if bit == 0 { '0' as u8 } else { '1' as u8 });
       }
       self.m_rank += 1;
       Some(text)
    }
}

/// iterates over all conjugates of a string of the form
/// S[i..|S|]S[0..i] for a string S
pub struct ConjugateIterator<'a, C : Eq + Copy + Clone> {
   m_text : &'a [C],
   m_pos : usize,
}

impl<'a, C : Eq + Copy + Clone> ConjugateIterator<'a, C> {
    #[allow(dead_code)]
    pub fn new(text: &'a [C]) -> ConjugateIterator<'a,C> {
        ConjugateIterator::<'a, C> { m_text : text, m_pos: 0 }
    }
}

impl<'a, C : Eq + Copy + Clone> Iterator for ConjugateIterator<'a, C> {
    type Item = Vec<C>;

    fn next(&mut self) -> Option<Vec<C>> { 
       if self.m_pos == self.m_text.len() { return None; }

       let mut ret = Vec::new();
       ret.reserve_exact(self.m_text.len());
       for i in self.m_pos..self.m_text.len() {
          ret.push(self.m_text[i]);
       }
       for i in 0..self.m_pos {
          ret.push(self.m_text[i]);
       }
       self.m_pos += 1;
       Some(ret)
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


/// converts &Option<String> to Option<&str>
pub fn stringopt_stropt(i : &Option<String>) -> Option<&str> {
    match i {
        None => None,
        Some(s) => Some(s.as_str())
    }
}


pub fn get_filename(o : &Option<String>) -> &str {
    match o {
	None => "none",
	Some(s) => s
    }
}


