#[macro_use] extern crate more_asserts;
extern crate log;
extern crate succinct;
use log::{debug, log_enabled, info, Level};

mod core;
mod io;

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

// #[derive(Debug)]
// struct SuffixEdge<'a> {
//     parent : &'a RefCell<LCPInterval>,
//     label : u8,
//     child : &'a RefCell<LCPInterval>,
// }
#[derive(Debug)]
struct SuffixEdge {
    parent : RefCell<LCPInterval>,
    label : u8,
    child : RefCell<LCPInterval>,
}


fn lcp_intervals(text: &[u8], sa :&[i32], lcp: &[u32]) -> Vec<LCPInterval> {
    
    let n = text.len();
    // let root = LCPInterval { depth : 0, begin : 0, end: (n-1) as u32};
    // let mut leaf = LCPInterval { depth : n as u32 - (sa[0] as u32), begin : 0, end : 0 };
    // let path = vec![&root, &leaf];

    let mut lcpintervals = vec![
        Rc::new(RefCell::new( LCPInterval { depth : 0, begin : 0, end: (n-1) as u32} )), 
        Rc::new(RefCell::new( LCPInterval { depth : n as u32 - (sa[0] as u32) - 1, begin : 0, end : 0 } ))];
    let mut path = vec![Rc::clone(&mut lcpintervals[0]), Rc::clone(&mut lcpintervals[1])];
    // let mut leaf = lcpintervals.last().unwrap();

    // let mut edges = Vec::new();

    for i in 1..n {
        let mut child_ptr : Option<Rc<RefCell<LCPInterval>>> = None;
        while lcp[i] < path.last().unwrap().borrow().depth {
            let node = Rc::clone(&path.pop().unwrap());
            (*(*node).borrow_mut()).end = (i - 1) as u32;
            // RefCell::get_mut(*node).end = (i - 1) as u32;
            
            // if let Some(child) = &child_ptr {
            //     let label = text[sa[child.borrow().begin as usize] as usize + node.borrow().depth as usize];
            //     edges.push( SuffixEdge { parent : RefCell::clone(&node), label, child : RefCell::clone(&child) });
            // }
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
        // if child_ptr.is_some() {
        //     let child  = Rc::clone(&child_ptr.unwrap());
        //     let label = text[sa[child.borrow().begin as usize] as usize + path.last().unwrap().borrow().depth as usize];
        //     edges.push( SuffixEdge { parent : Rc::clone(&path.last().unwrap()), label, child });
        // }
        // create a new leaf for index i
        assert_lt!(sa[i] as u32 + n as u32 - (sa[i] as u32) - 1, n as u32);
        lcpintervals.push(Rc::new(RefCell::new( LCPInterval { depth : n as u32 - (sa[i] as u32) - 1, begin : i as u32, end : i as u32})));
        path.push(Rc::clone(lcpintervals.last().unwrap()));
    }
    // let mut child_ptr : Option<Rc<RefCell<LCPInterval>>> = None;
    // treat remaining nodes on `path`
    while !path.is_empty() {
        let node = path.pop().unwrap();
        node.borrow_mut().end = (n - 1) as u32;
        // if let Some(child)  = &child_ptr {
        //     let label = text[sa[child.borrow().begin as usize] as usize + node.borrow().depth as usize];
        //     edges.push( SuffixEdge { parent : Rc::clone(&node), label, child : Rc::clone(child) } );
        // }
        // child_ptr = Some(Rc::clone(&node));
    }
    let mut ret = Vec::new();
    for interval in lcpintervals {
        ret.push((*(*interval).borrow()).clone());
        // ret.push(*RefCell::as_ptr(interval));
        assert_lt!(ret.last().unwrap().begin as usize, n);
        assert_lt!(ret.last().unwrap().end as usize, n);
        assert_lt!(sa[ret.last().unwrap().begin as usize] as usize + ret.last().unwrap().depth as usize, n);
    }
    ret
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

fn main() {
    let args = Args::parse();
    // // print!("{:?}", args.attractor);
    // // return;
    // use std::sync::Arc;
    // let a= Rc::new(RefCell::new(5));
    // let mut b = a.clone();
    // *(*b).borrow_mut() = 6;
    // println!("a={}", a.borrow());
    // return;

    env_logger::init();

    info!("prefixlength: {}", args.prefixlength);

    info!("read text");
    // let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);
    let text = {
        let mut text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);
        text.push(0u8);
        text
    };

    let n = text.len();

    for attractor_position in args.attractor.as_slice() {
        assert_lt!(*attractor_position as usize, n);
    }

    let sa = { 
        let mut sa = vec![0; text.len()];
        cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
        sa
    };
    // let isa = core::inverse_permutation(&sa.as_slice());
    let lcp = {
        let phi = core::compute_phi(&sa.as_slice());
        let plcp = core::compute_plcp(&text.as_slice(), &phi.as_slice());
        core::compute_lcp(&plcp.as_slice(), &sa.as_slice())
    };
    let lcpintervals = lcp_intervals(&text, &sa, &lcp);
    

    use succinct::*;
    use succinct::BitVector;
    // use succinct::Select1Support;
    use succinct::BinSearchSelect;
    use succinct::bit_vec::BitVecMut;

    let attractor_positions = {
        let mut v = BitVector::with_fill(n as u64, false);
        for s in args.attractor.into_iter() {
            v.set_bit(s, true);
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
    for lcpinterval in lcpintervals {
        if lcpinterval.depth == 0 {
            continue
        } 
        let rmq = d_rmq.query(lcpinterval.begin as usize, lcpinterval.end as usize + 1);
        if lcpinterval.depth as u64 <= rmq {
            is_attractor = false;
            let startpos = sa[lcpinterval.begin as usize] as usize;
            let endpos = std::cmp::min(sa[lcpinterval.begin as usize] as usize+lcpinterval.depth as usize, n as usize);
            println!("substring '{}' not covered!", str::from_utf8(&text[startpos..endpos]).unwrap());
        }
    }
    if is_attractor {
        println!("valid attractor");
    }
}
