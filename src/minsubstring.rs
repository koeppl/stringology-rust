extern crate log;
use log::{debug, log_enabled, info, Level};

mod core;
mod io;

use std::cell::RefCell;

/// represents a suffix array node as as an interval in the SA/LCP array
#[derive(Debug,Copy,Clone)]
struct LCPInterval {
    /// string depth of the node
    depth : u32,
    /// first suffix array position
    begin : u32,
    /// last suffix array position
    end : u32,
}

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
        RefCell::new( LCPInterval { depth : 0, begin : 0, end: (n-1) as u32} ), 
        RefCell::new( LCPInterval { depth : n as u32 - (sa[0] as u32), begin : 0, end : 0 } )];
    let mut path = vec![RefCell::clone(&mut lcpintervals[0]), RefCell::clone(&mut lcpintervals[1])];
    // let mut leaf = lcpintervals.last().unwrap();

    let mut edges = Vec::new();

    for i in 1..n {
        let mut child_ptr : Option<RefCell<LCPInterval>> = None;
        while lcp[i] < path.last().unwrap().borrow().depth {
            let mut node = RefCell::clone(&path.pop().unwrap());
            RefCell::get_mut(&mut node).end = (i - 1) as u32;
            
            if let Some(child) = &child_ptr {
                let label = text[sa[child.borrow().begin as usize] as usize + node.borrow().depth as usize];
                edges.push( SuffixEdge { parent : RefCell::clone(&node), label, child : RefCell::clone(&child) });
            }
            child_ptr = Some(RefCell::clone(&node));
        }
        if lcp[i] > path.last().unwrap().borrow().depth {
            assert!(child_ptr.is_some());
            // create internal node
            if let Some(child) = &child_ptr {
                let child_begin = child.borrow().begin;
                lcpintervals.push(RefCell::new( LCPInterval { depth : lcp[i], begin : child_begin, end : n as u32 } ));
                path.push(RefCell::clone(lcpintervals.last().unwrap()));
            }
        }
        if child_ptr.is_some() {
            let child  = RefCell::clone(&child_ptr.unwrap());
            let label = text[sa[child.borrow().begin as usize] as usize + path.last().unwrap().borrow().depth as usize];
            edges.push( SuffixEdge { parent : RefCell::clone(&path.last().unwrap()), label, child });
        }
        // create a new leaf for index i
        lcpintervals.push(RefCell::new( LCPInterval { depth : n as u32 - (sa[i] as u32), begin : i as u32, end : i as u32}));
        path.push(RefCell::clone(lcpintervals.last().unwrap()));
    }
    let mut child_ptr : Option<RefCell<LCPInterval>> = None;
    // treat remaining nodes on `path`
    while !path.is_empty() {
        let node = path.pop().unwrap();
        node.borrow_mut().end = (n - 1) as u32;
        if let Some(child)  = &child_ptr {
            let label = text[sa[child.borrow().begin as usize] as usize + node.borrow().depth as usize];
            edges.push( SuffixEdge { parent : RefCell::clone(&node), label, child : RefCell::clone(child) } );
        }
        child_ptr = Some(RefCell::clone(&node));
    }
    let mut ret = Vec::new();
    for interval in lcpintervals {
        unsafe {
        ret.push(*RefCell::as_ptr(&interval));
    }
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

}

fn main() {
    let args = Args::parse();

    env_logger::init();

    info!("prefixlength: {}", args.prefixlength);

    info!("read text");
    let text = io::file_or_stdin2byte_vector(core::stringopt_stropt(&args.infilename), args.prefixlength);

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
    println!("{:?}", lcp_intervals(&text, &sa, &lcp));
}
