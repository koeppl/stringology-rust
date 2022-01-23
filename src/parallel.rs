#[allow(dead_code)] mod common;
#[macro_use] extern crate more_asserts;

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
    factors.push(n);
	println!("duval factors {:?}", factors);

    struct Conjugate {
        index : usize, //@ conjugate number
        prev_border : usize,
        next_border : usize
    }
    let mut conjugates = Vec::new();

	for	factor in 0..factors.len()-1 {
        for j in factors[factor]..factors[factor+1] {
            conjugates.push(Conjugate { 
                index : j-factors[factor], 
                prev_border : factors[factor], 
                next_border : factors[factor+1] });
        }
	}
    conjugates.sort_by(|a, b| {
        let mut len = 0;
        let alen = a.next_border-a.prev_border;
        let blen = b.next_border-b.prev_border;
        while len < alen*blen {
            let char_a = text[a.prev_border + ((a.index+len) % alen )];
            let char_b = text[b.prev_border + ((b.index+len) % blen )];
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
        bbwt[i] = if conjugates[i].index == 0 { text[conjugates[i].next_border - 1] } else { text[conjugates[i].prev_border + conjugates[i].index-1] };
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


fn main() {
    let n = 100;
    for number in 1..n {
        use std::str;

        let text = format!("{:b}", number);
        // let sa = suffixarray_naive(&text.as_bytes());
        // let bwt = bwt_from_sa(&text.as_bytes(), &sa);
        let bwt = compute_bwt_matrix(&text.as_bytes());
        let bbwt = bbwt(&text.as_bytes());
		println!("text {:?}", text);
		println!("bwt  {:?}", str::from_utf8(&bwt).unwrap());
		println!("bbwt {:?}", str::from_utf8(&bbwt).unwrap());
        println!("{:?} {} {}", bwt, common::number_of_runs(&mut bwt.as_slice()), common::number_of_runs(&mut bbwt.as_slice()));
    }
}
