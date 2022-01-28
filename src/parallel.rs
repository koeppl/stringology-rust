#[allow(dead_code)] mod io;
#[allow(dead_code)] mod core;
#[macro_use] extern crate more_asserts;


/**
 * the border array B of text[0..n) of length n+1
 * It uses a dummy 0 value at the beginning (often -1 in the literature)
 * to speed up computation by avoiding an additional if instruction.
 * B[i] stores the longest prefix of text that is a suffix of text[1..i).
 * Hence, B[0] and B[1] are dummy values having to meaning.
 *
 */
fn border_array<C : Eq>(text: &[C]) -> Vec<usize> {
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

#[test]
fn test_border_array() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let border = border_array(&text);
       assert_eq!(border[0],0);
       assert_eq!(border[1],0);
       for i in 2..text.len() {
          for b in 0..border[i] {
             assert_eq!(text[b], text[i-border[i]+b]);
          }
       }
    }
}



// test cases: aaaaaaa -> 012345 ...
// ababab -> 0022345...

/// the smallest period of a string is given by the  last entry of its border array
fn smallest_period(border_array : &[usize]) -> usize {
    let n = border_array.len()-1;
    n - border_array[n]
}


#[test]
fn test_period() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let border = border_array(&text);
       let period = smallest_period(&border);
       for i in 0..text.len()-period {
          assert_eq!(text[i], text[i+period]);
       }
    }
}


fn lyndon_conjugate<C : Ord + Copy + Clone>(text: &[C]) -> usize {
   let n = text.len();
   let mut doubletext = Vec::new();
   doubletext.extend_from_slice(text);
   doubletext.extend_from_slice(text); //@ TODO: instead of doubling the text, we could write a wrapper around the text

   let mut factors = core::duval(&doubletext);
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

#[test]
fn test_lyndon_conjugate() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let n = text.len();
       let lconjugate = lyndon_conjugate(&text);
       assert_lt!(lconjugate, n);
       for conj in 0..text.len() {
          if conj == lconjugate { continue;}
          for len in 0..text.len() {
             let lcharpos = (lconjugate + len) % n;
             let ccharpos = (conj + len) % n;
             let lchar = text[lcharpos];
             let cchar = text[ccharpos];
             if lchar < cchar { break; }
             assert_eq!(lchar, cchar); //@ otherwise, lconjugate is not the lex. smallest conjugate!
          }
       }
    }
}

/// computes the BWT based on the matrix, i.e., the sorting of the cyclic conjugates of the 
/// input text by first finding its Lyndon conjugate, appending a 0 byte, 
/// then computing the BWT of this conjugate
/// via the suffix array, and removing the 0 byte at the end.
fn bwt_by_matrix(text: &[u8]) -> Vec<u8> {
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
    let mut bwt = core::bwt_from_text_by_sa(&newtext);
    bwt.remove(bwt.iter().position(|&x| x == 0).unwrap());
    bwt
}

pub const MAX_TEST_ITER : usize = 4096;
#[test]
fn test_bwt_by_matrix() {
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
        if text.len() < 2 { continue; }
        let naive = compute_bwt_matrix(&text[0..text.len()-1]);
        let clever = bwt_by_matrix(&text[0..text.len()-1]);
        if naive != clever {
            bwt_by_matrix(&text[0..text.len()-1]);
        }
        assert_eq!(naive, clever);
    }
}

/// a string is primitive if it is not the x-times concatenation of a string, for x being an
/// integer >= 2
fn is_primitive<C : Eq>(text: &[C]) -> bool {
    let border = border_array(text);
    let period = smallest_period(&border);
    if period == text.len() {
       return true;
    }
    (text.len() % smallest_period(&border)) != 0
}

fn bbwt<C: Ord + Clone + Copy>(text: &[C]) -> Vec<C> {
    let n = text.len();
    let factors = core::duval(&text);
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
fn suffixarray_naive<C : Ord>(text: &[C]) -> Vec<usize> {
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
fn bwt_from_sa<C : Clone + Copy>(text: &[C], sa: &Vec<usize>) -> Vec<C> {
    let n = text.len();
    let mut bwt = vec![text[0]; n];
    for i in 0..n {
        bwt[i] = text[(n + (sa[i] as usize)-1)  % n];
    }
    bwt
}

/// computes the rightmost column of the BWT matrix
/// note that this is a O(n^2 lg n) algorithm!
fn compute_bwt_matrix<T : std::cmp::Ord + Copy>(text: &[T]) -> Vec<T> {
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
pub struct LyndonWordFactory {
   m_length : usize,
   m_alphabet_size : usize,
   m_stack : Vec<u8> 
}

impl LyndonWordFactory {
    #[allow(dead_code)]
    pub fn new(length : usize, alphabet_size : usize) -> LyndonWordFactory {
        LyndonWordFactory { m_length : length, m_alphabet_size : alphabet_size, m_stack : Vec::new() }
    }
}

impl Iterator for LyndonWordFactory {
    type Item = Vec<u8>;

    /// Copied from David Eppstein @ https://www.ics.uci.edu/~eppstein/PADS/Lyndon.py
    /// Generate nonempty Lyndon words of length <= n over an s-symbol alphabet.
    /// The words are generated in lexicographic order, using an algorithm from
    /// J.-P. Duval, Theor. Comput. Sci. 1988, doi:10.1016/0304-3975(88)90113-2.
    /// As shown by Berstel and Pocchiola, it takes constant average time
    /// per generated word.
    fn next(&mut self) -> Option<Vec<u8>> { 
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
          if stack.is_empty() { return None; }
          else {
             *stack.last_mut().unwrap() += 1;                      //@ increment the last non-z symbol
             return Some(stack.clone());
          }
       }
       return None;
    }
}

pub struct BinaryStringFactory {
   m_length : u8,
   m_rank : u64,
}

impl BinaryStringFactory {
    #[allow(dead_code)]
    pub fn new(length : u8) -> BinaryStringFactory {
        BinaryStringFactory { m_length : length, m_rank : 0 }
    }
}

impl Iterator for BinaryStringFactory {
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


fn second_criteria(newtext: &[u8], oldtext: &[u8]) -> bool {
    // return zero_order_entropy(newtext.into_iter()) < zero_order_entropy(oldtext.into_iter());
   return newtext.into_iter().filter(|&c| *c == '0' as u8).count() < oldtext.into_iter().filter(|&c| *c == '0' as u8).count();
}

fn binary_vector_to_str(text : &[u8]) -> String {
   let mut output = String::new();
   for c in text {
      if *c == 0 { output.push('a'); } else { output.push('b'); }
   }
   output
}

fn main() {

   const max_length : usize = 40;

   let mut num_lyndon_words = [0; max_length];
   let mut bwt_win_counter =[0;max_length]; 
   let mut bbwt_win_counter =[0;max_length]; 
   let mut tie_win_counter =[0;max_length]; 

   // let mut score_counter = 0i64; // same, but stores the diff
   // let mut best_bbwt_text : [Vec<u8>; max_length] = Default::default();
   let mut best_bbwt_text = std::iter::repeat(vec![]).take(max_length).collect::<Vec<_>>();
   let mut best_bbwt_bbwtrun = [0;max_length];
   let mut best_bbwt_bwtrun = [0;max_length];


   let mut total_counter = 0;

    for text in LyndonWordFactory::new(max_length, 2) {
       {
          // println!("fact = {:?}", text);
          let bwt = compute_bwt_matrix(&text);
          let bwt_runs = core::number_of_runs(&mut bwt.as_slice());

          assert_gt!(text.len(), 0);
          let lindex = text.len()-1;
          assert_lt!(lindex, max_length);
          for conjugate in ConjugateIterator::new(&text) {
             // println!("conj = {:?}", conjugate);
             // assert_eq!(bwt, compute_bwt_matrix(&conjugate));  
             let bbwt = bbwt(&conjugate);
             let bbwt_runs = core::number_of_runs(&mut bbwt.as_slice());

             if bwt_runs < bbwt_runs { bwt_win_counter[lindex] += 1; } else if bwt_runs > bbwt_runs { bbwt_win_counter[lindex] += 1 } else { tie_win_counter[lindex] += 1 };
             if bbwt_runs < bwt_runs && 
                (bwt_runs-bbwt_runs > (best_bbwt_bwtrun[lindex]-best_bbwt_bbwtrun[lindex]) ||
                 (bwt_runs-bbwt_runs == (best_bbwt_bwtrun[lindex]-best_bbwt_bbwtrun[lindex]) && second_criteria(&conjugate, & best_bbwt_text[lindex].as_slice()))) {
                   best_bbwt_bwtrun[lindex] = bwt_runs;
                   best_bbwt_bbwtrun[lindex] = bbwt_runs;
                   best_bbwt_text[lindex].clear();
                   best_bbwt_text[lindex].extend_from_slice(&conjugate);
             }

          }
          num_lyndon_words[lindex] += 1;
          total_counter += 1;
       }
       if total_counter % 100 == 0 {
          use std::str;
          for length in 0..max_length {
             println!("length={} lyndon_words={} bwt_wins={} bbwt_wins={} ties={}", length, num_lyndon_words[length], bwt_win_counter[length], bbwt_win_counter[length], tie_win_counter[length]);
             // println!("length={} text={} bwt_runs={} bbwt_runs={}", length, str::from_utf8(&best_bbwt_text[length]).unwrap(), best_bbwt_bwtrun[length], best_bbwt_bbwtrun[length]);
             println!("length={} text={} bwt_runs={} bbwt_runs={}", length, binary_vector_to_str(&best_bbwt_text[length]), best_bbwt_bwtrun[length], best_bbwt_bbwtrun[length]);
          }
       }
    }

    // for k in 2..20 {
    //     let mut text = Vec::new();
    //     text.push(1u8);
    //     text.push(1u8);
    //     for _ in 0..k {
    //         text.push(0u8);
    //     }
    //     text.push(1u8);
    //     for _ in 0..k-1 {
    //         text.push(0u8);
    //     }
    //     text.push(1u8);
    //     let bwt = compute_bwt_matrix(&text);
    //     let bbwt = bbwt(&text);
    //     let bwt_runs = io::number_of_runs(&mut bwt.as_slice());
    //     let bbwt_runs = io::number_of_runs(&mut bbwt.as_slice());
    //     // println!("text={} bwt_runs={} bbwt_runs={}", str::from_utf8(&text.slice()).unwrap(), bwt_runs, bbwt_runs);
    //     println!("bwt_runs={} bbwt_runs={}", bwt_runs, bbwt_runs);
    // }


    // for number_of_bits in 3..36 {
    //     let mut bwt_win_counter =0u64; 
    //     let mut bbwt_win_counter =0u64; 
    //     let mut tie_win_counter =0u64; 
    //     let mut score_counter = 0i64; // same, but stores the diff
    //     let mut best_bbwt_text = Vec::new();
    //     let mut best_bbwt_bbwtrun = 0;
    //     let mut best_bbwt_bwtrun = 0;
    //     let mut primitive_words = 0;
    //     for number in 0..(1<<number_of_bits) {
    //         let mut text = Vec::new();
    //         text.reserve(number_of_bits);
    //         for i in 0..number_of_bits {
    //             let bit = number & (1<<i);
    //             text.push( if bit == 0 { '0' as u8 } else { '1' as u8 });
    //         }
    //         // let formatnumber = 
    //         // let text = format!(formatstring, number);
    //
    //         // println!("text={:?} border_array={:?} primitive?={}", text, border_array(&text), is_primitive(&text));
    //
    //         if is_primitive(&text) {
    //             primitive_words += 1;
    //         }
    //
    //         // let sa = suffixarray_naive(&text.as_bytes());
    //         // let bwt = bwt_from_sa(&text.as_bytes(), &sa);
    //         let bwt = compute_bwt_matrix(&text);
    //         let bbwt = bbwt(&text);
    //         let bwt_runs = core::number_of_runs(&mut bwt.as_slice());
    //         let bbwt_runs = core::number_of_runs(&mut bbwt.as_slice());
    //         score_counter += bbwt_runs as i64 - bwt_runs as i64;
    //         if bwt_runs < bbwt_runs { bwt_win_counter += 1; } else if bwt_runs > bbwt_runs { bbwt_win_counter += 1 } else { tie_win_counter += 1 };
    //
    //         if bbwt_runs < bwt_runs && 
    //             (bwt_runs-bbwt_runs > (best_bbwt_bwtrun-best_bbwt_bbwtrun) ||
    //              (bwt_runs-bbwt_runs == (best_bbwt_bwtrun-best_bbwt_bbwtrun) && second_criteria(&text, & best_bbwt_text.as_slice()))) {
    //                 best_bbwt_bwtrun = bwt_runs;
    //                 best_bbwt_bbwtrun = bbwt_runs;
    //                 best_bbwt_text.clear();
    //                 best_bbwt_text.extend_from_slice(&text);
    //         }
    //     }
    //
    //     use std::str;
    //     println!("bits={} nonprimitive_words={} bwt_wins={} bbwt_wins={} ties={} score={}", number_of_bits, (1<<number_of_bits)-primitive_words, bwt_win_counter, bbwt_win_counter, tie_win_counter, score_counter);
    //     println!("bits={} text={} bwt_runs={} bbwt_runs={}", number_of_bits, str::from_utf8(&best_bbwt_text).unwrap(), best_bbwt_bwtrun, best_bbwt_bbwtrun);
    //     // println!("text {:?}", text);
    //     // println!("bwt  {:?}", str::from_utf8(&bwt).unwrap());
    //     // println!("bbwt {:?}", str::from_utf8(&bbwt).unwrap());
    //     // println!("{:?} {} {}", bwt, io::number_of_runs(&mut bwt.as_slice()), io::number_of_runs(&mut bbwt.as_slice()));
    //     // if (number).leading_zeros() as i32 - (number+1).leading_zeros() as i32  > 0  {
    //     // }
    // }
}


// TODO: compute the Lyndon factor in Duval of (TT) that spans over the border between T and T,
// which has the length n (or it is exactly T). This is the Lyndon conjugate L of T
// Then the BWT and the BBWT of L are identical
// In particular, the matrix-based BWT and the SA-based BWT of L should be the same when removing
// the $
// TODO: https://www.ics.uci.edu/~eppstein/PADS/Lyndon.py
