#[allow(dead_code)] mod common;
#[macro_use] extern crate more_asserts;


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

/// smallest period can be obtained by the last entry of the border array
fn smallest_period(border_array : &[usize]) -> usize {
    let n = border_array.len()-1;
    n - border_array[n]
}

fn is_primitive<C : Eq>(text: &[C]) -> bool {
    let border = border_array(text);
    let period = smallest_period(&border);
    if period == text.len() {
       return true;
    }
    (text.len() % smallest_period(&border)) != 0
}

//@TODO: needs to be included!
fn duval<C : Ord>(text: &[C]) -> Vec<usize> {
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

fn bbwt<C: Ord + Clone + Copy>(text: &[C]) -> Vec<C> {
    let n = text.len();
    let mut factors = duval(&text);
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

fn zero_order_entropy<'a, I: Iterator<Item = &'a u8>>(text_iter : I) -> f64 {
    let mut char_counters : Vec<usize> = vec![0; std::u8::MAX as usize + 1]; 
    let mut total_count = 0;
    for c in text_iter {
        let index : usize = (*c).into();
        char_counters[index] += 1;
        total_count += 1;
    }
    let mut sum = 0 as f64;
    for count in char_counters {
        if count > 0 {
            sum += (count as f64) * ((total_count as f64 / count as f64).log2());
        }
    }
    sum / (total_count as f64)
}

fn second_criteria(newtext: &[u8], oldtext: &[u8]) -> bool {
    // return zero_order_entropy(newtext.into_iter()) < zero_order_entropy(oldtext.into_iter());
   return newtext.into_iter().filter(|&c| *c == '0' as u8).count() < oldtext.into_iter().filter(|&c| *c == '0' as u8).count();
}

fn main() {

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
    //     let bwt_runs = common::number_of_runs(&mut bwt.as_slice());
    //     let bbwt_runs = common::number_of_runs(&mut bbwt.as_slice());
    //     // println!("text={} bwt_runs={} bbwt_runs={}", str::from_utf8(&text.slice()).unwrap(), bwt_runs, bbwt_runs);
    //     println!("bwt_runs={} bbwt_runs={}", bwt_runs, bbwt_runs);
    // }


    for number_of_bits in 3..36 {
        let mut bwt_win_counter =0u64; 
        let mut bbwt_win_counter =0u64; 
        let mut tie_win_counter =0u64; 
        let mut score_counter = 0i64; // same, but stores the diff
        let mut best_bbwt_text = Vec::new();
        let mut best_bbwt_bbwtrun = 0;
        let mut best_bbwt_bwtrun = 0;
        let mut primitive_words = 0;
        for number in 0..(1<<number_of_bits) {
            let mut text = Vec::new();
            text.reserve(number_of_bits);
            for i in 0..number_of_bits {
                let bit = number & (1<<i);
                text.push( if bit == 0 { '0' as u8 } else { '1' as u8 });
            }
            // let formatnumber = 
            // let text = format!(formatstring, number);

            // println!("text={:?} border_array={:?} primitive?={}", text, border_array(&text), is_primitive(&text));

            if is_primitive(&text) {
                primitive_words += 1;
            }

            // let sa = suffixarray_naive(&text.as_bytes());
            // let bwt = bwt_from_sa(&text.as_bytes(), &sa);
            let bwt = compute_bwt_matrix(&text);
            let bbwt = bbwt(&text);
            let bwt_runs = common::number_of_runs(&mut bwt.as_slice());
            let bbwt_runs = common::number_of_runs(&mut bbwt.as_slice());
            score_counter += bbwt_runs as i64 - bwt_runs as i64;
            if bwt_runs < bbwt_runs { bwt_win_counter += 1; } else if bwt_runs > bbwt_runs { bbwt_win_counter += 1 } else { tie_win_counter += 1 };

            if bbwt_runs < bwt_runs && 
                (bwt_runs-bbwt_runs > (best_bbwt_bwtrun-best_bbwt_bbwtrun) ||
                 (bwt_runs-bbwt_runs == (best_bbwt_bwtrun-best_bbwt_bbwtrun) && second_criteria(&text, & best_bbwt_text.as_slice()))) {
                    best_bbwt_bwtrun = bwt_runs;
                    best_bbwt_bbwtrun = bbwt_runs;
                    best_bbwt_text.clear();
                    best_bbwt_text.extend_from_slice(&text);
            }
        }

        use std::str;
        println!("bits={} nonprimitive_words={} bwt_wins={} bbwt_wins={} ties={} score={}", number_of_bits, (1<<number_of_bits)-primitive_words, bwt_win_counter, bbwt_win_counter, tie_win_counter, score_counter);
        println!("bits={} text={} bwt_runs={} bbwt_runs={}", number_of_bits, str::from_utf8(&best_bbwt_text).unwrap(), best_bbwt_bwtrun, best_bbwt_bbwtrun);
        // println!("text {:?}", text);
        // println!("bwt  {:?}", str::from_utf8(&bwt).unwrap());
        // println!("bbwt {:?}", str::from_utf8(&bbwt).unwrap());
        // println!("{:?} {} {}", bwt, common::number_of_runs(&mut bwt.as_slice()), common::number_of_runs(&mut bbwt.as_slice()));
        // if (number).leading_zeros() as i32 - (number+1).leading_zeros() as i32  > 0  {
        // }
    }
}
