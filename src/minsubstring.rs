#[macro_use] extern crate more_asserts;
extern crate log;
extern crate succinct;
use log::{debug, log_enabled, info, Level};

mod core;
mod io;
mod fibonacci;

use std::cell::RefCell;


/// represents a suffix array node as as an interval in the SA/LCP array
#[derive(Debug,Clone)]
struct LCPInterval {
    /// string depth of the node
    depth : u32,
    /// first suffix array position
    begin : u32,
    /// last suffix array position
    end : u32,
}
use std::rc::Rc;

/// represents an edge in the suffix tree
#[derive(Debug)]
struct SuffixEdge {
    parent : Rc<RefCell<LCPInterval>>,
    label : u8,
    child : Rc<RefCell<LCPInterval>>,
}


fn lcp_intervals(text: &[u8], sa :&[i32], lcp: &[u32]) -> Vec<SuffixEdge> {
    
    let n = text.len();
    // let root = LCPInterval { depth : 0, begin : 0, end: (n-1) as u32};
    // let mut leaf = LCPInterval { depth : n as u32 - (sa[0] as u32), begin : 0, end : 0 };
    // let path = vec![&root, &leaf];

    let mut lcpintervals = vec![
        Rc::new(RefCell::new( LCPInterval { depth : 0, begin : 0, end: (n-1) as u32} )), 
        Rc::new(RefCell::new( LCPInterval { depth : n as u32 - (sa[0] as u32) - 1, begin : 0, end : 0 } ))];
    let mut path = vec![Rc::clone(&mut lcpintervals[0]), Rc::clone(&mut lcpintervals[1])];
    // let mut leaf = lcpintervals.last().unwrap();

    let mut edges = Vec::new();

    for i in 1..n {
        let mut child_ptr : Option<Rc<RefCell<LCPInterval>>> = None;
        while lcp[i] < path.last().unwrap().borrow().depth {
            let node = Rc::clone(&path.pop().unwrap());
            let mut node_ref = (*node).borrow_mut();
            (*node_ref).end = (i - 1) as u32;

            if let Some(child) = &child_ptr {
                let label = text[sa[child.borrow().begin as usize] as usize + (*node_ref).depth as usize];
                edges.push( SuffixEdge { parent : Rc::clone(&node), label, child : Rc::clone(&child) });
            }
            child_ptr = Some(Rc::clone(&node));
        }
        if lcp[i] > path.last().unwrap().borrow().depth {
            assert!(child_ptr.is_some());
            // create internal node
            if let Some(child) = &child_ptr {
                let child_begin = child.borrow().begin;
                assert_lt!(lcp[i] as usize + sa[child_begin as usize] as usize, n);
                lcpintervals.push(Rc::new(RefCell::new( LCPInterval { depth : lcp[i], begin : child_begin, end : n as u32 }) ));
                path.push(Rc::clone(lcpintervals.last().unwrap()));
            }
        }
        if child_ptr.is_some() {
            let child  = Rc::clone(&child_ptr.unwrap());
            let label = text[sa[child.borrow().begin as usize] as usize + path.last().unwrap().borrow().depth as usize];
            edges.push( SuffixEdge { parent : Rc::clone(&path.last().unwrap()), label, child });
        }
        // create a new leaf for index i
        assert_lt!(sa[i] as u32 + n as u32 - (sa[i] as u32) - 1, n as u32);
        lcpintervals.push(Rc::new(RefCell::new( LCPInterval { depth : n as u32 - (sa[i] as u32) - 1, begin : i as u32, end : i as u32})));
        path.push(Rc::clone(lcpintervals.last().unwrap()));
    }
    let mut child_ptr : Option<Rc<RefCell<LCPInterval>>> = None;
    // treat remaining nodes on `path`
    while !path.is_empty() {
        let node = path.pop().unwrap();
        node.borrow_mut().end = (n - 1) as u32;
        if let Some(child)  = &child_ptr {
            let label = text[sa[child.borrow().begin as usize] as usize + node.borrow().depth as usize];
            edges.push( SuffixEdge { parent : Rc::clone(&node), label, child : Rc::clone(child) } );
        }
        child_ptr = Some(Rc::clone(&node));
    }
    // let mut ret = Vec::new();

    #[cfg(debug_assertions)]
    for interval in lcpintervals {
        let interval = (*(*interval).borrow()).clone();
        // ret.push((*(*interval).borrow()).clone());
        // ret.push(*RefCell::as_ptr(interval));
        assert_lt!(interval.begin as usize, n);
        assert_lt!(interval.end as usize, n);
        assert_lt!(sa[interval.begin as usize] as usize + interval.depth as usize, n);
    }

    edges
}









extern crate clap;
use clap::Parser;
/// computes the BWT via divsufsort
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

   /// the input file to read (otherwise read from stdin)
   #[arg(short, long)]
   infilename: Option<String>,

   // /// the output file to write (otherwise write from stdout)
   // #[arg(short, long)]
   // outfilename: Option<String>,

   /// the length of the prefix to parse
   #[arg(short, long, default_value_t = 0)]
   prefixlength: usize,

   /// string attractor
   #[arg(short, long,num_args(1..))]
   attractor : Vec<u64>,
}

fn is_attractor(text : &[u8], attractor: &[u64]) -> bool {
    assert_gt!(text.len(), 0);
    assert_eq!(*text.last().unwrap(), 0u8);
    let n = text.len();
    for &attractor_position in attractor {
        assert_lt!(attractor_position as usize, n);
    }


    let sa = { 
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };
    // let isa = core::inverse_permutation(&sa.as_slice());
    let lcp = {
        let phi = core::compute_phi(&sa.as_slice());
        let plcp = core::compute_plcp(&text, &phi.as_slice());
        core::compute_lcp(&plcp.as_slice(), &sa.as_slice())
    };
    let suffix_edges = lcp_intervals(&text, &sa, &lcp);
    

    use succinct::*;
    use succinct::BitVector;
    // use succinct::Select1Support;
    use succinct::BinSearchSelect;
    use succinct::bit_vec::BitVecMut;

    let attractor_positions = {
        let mut v = BitVector::with_fill(n as u64, false);
        for s in attractor.into_iter() {
            v.set_bit(*s, true);
        }
        v
    };
    use succinct::Rank9;
    //TODO: remove clone() calls!
    let rank = Rank9::new(attractor_positions.clone());
    let select = BinSearchSelect::new(rank.clone()); //@ starts with index 0
    
    // for i in 0..n {
    //     println!("rank {} -> {}", i, rank.rank1(i as u64));
    //     if let Some(pos) = select.select1(i as u64) {
    //         println!("select {} -> {:?}",i,  pos);
    //     }
    // }


    let mut arr_d = vec!(0; n);
    for i in 0..n {
        let text_position = sa[i] as u64;
        if attractor_positions.get_bit(text_position as u64) == false {
            let successor_rank = rank.rank1(text_position as u64);
            match select.select1(successor_rank) {
                Some(pos) =>  {
                    assert!(attractor_positions.get_bit(pos) == true);
                    arr_d[i] = pos - text_position as u64;
                },
                None => arr_d[i] = n as u64,
            }
        }
    }

    use segment_tree::SegmentPoint;
    use segment_tree::ops::Min;
    use std::str;
    let d_rmq = SegmentPoint::build(arr_d.clone(), Min);

    let mut is_attractor = true;
    for edge in suffix_edges {
        let lcplength = edge.parent.borrow().depth as usize +1;
        let lcpinterval = edge.child.borrow();
        info!("{:?}", lcpinterval);
        let rmq = d_rmq.query(lcpinterval.begin as usize, lcpinterval.end as usize + 1) as usize;
        if lcplength <= rmq {
            is_attractor = false;
            let startpos = sa[lcpinterval.begin as usize] as usize;
            let endpos = std::cmp::min(sa[lcpinterval.begin as usize] as usize + lcplength, n as usize);
            println!("substring '{}' not covered!", str::from_utf8(&text[startpos..endpos]).unwrap());
        }
    }
    is_attractor
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    info!("prefixlength: {}", args.prefixlength);

    let text = {
        let mut text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);
        text.push(0u8);
        text
    };
    let n = text.len();

    for attractor_position in args.attractor.as_slice() {
        if (*attractor_position as usize) >= n {
            eprintln!("specified attractor position {} is larger than text (length: {})", *attractor_position, n);
            std::process::exit(1);
        }
    }

    if !is_attractor(text.as_slice(), args.attractor.as_slice()) {
        println!("not a valid attractor");
            std::process::exit(2);
    } else {
        println!("valid attractor");
    }

}

mod perioddoubling;

/// Period Doubling Sequences have a string attractor of length 2.
/// Ref:
/// Luke Schaeffer, Jeffrey Shallit
/// String Attractors for Automatic Sequences. CoRR abs/2012.06840 (2020), https://arxiv.org/abs/2012.06840
#[test]
fn test_period_doubling() {
    for i in 5..16 {
        let mut text = perioddoubling::period_doubling_sequence(i);
        let attractor = [3 * (1<<(i-4)) - 1, 3 * (1<<(i-3)) - 1];
        println!("len={} -> attr = {:?}", text.len(), attractor);
        text.push(0u8);
        assert!(is_attractor(text.as_slice(), &attractor));
    }
}


/// The minimum string attractor of Sturmian words has been identified to be exactly of size two, 
/// where the positions are chosen directly at the boundary of the two previous recurrent
/// substrings.
/// Ref:
/// Sabrina Mantaci, Antonio Restivo, Giuseppe Romana, Giovanna Rosone, Marinella Sciortino:
/// String Attractors and Combinatorics on Words. ICTCS 2019: 57-71, http://ceur-ws.org/Vol-2504/paper8.pdf
#[test]
fn test_fibonacci_attractor() {
    for i in 3..16 {
        let attractor = [fibonacci::fibonacci_number(i-1) as u64 -1, fibonacci::fibonacci_number(i-1) as u64 -2];
        let mut text = fibonacci::fibonacci(i);
        text.push(0u8);
        assert!(is_attractor(text.as_slice(), &attractor));
    }
}
