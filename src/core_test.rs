#[allow(dead_code)] extern crate env_logger;
#[macro_use] extern crate more_asserts;
#[allow(dead_code)] mod io;
#[allow(dead_code)] mod core;

mod fibonacci;
extern crate log;
use log::{debug};

#[test]
fn test_duval() {
    pub const MAX_TEST_ITER : usize = 4096;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       
        let factors = core::duval(&text);

        let n = text.len();
        let sa = { 
            let mut sa = vec![0; n];
            assert!(!text[..text.len()-1].into_iter().any(|&x| x == 0));
            cdivsufsort::sort_in_place(&text, sa.as_mut_slice());
            sa
        };
        let isa = core::inverse_permutation(&sa.as_slice());
        if log::log_enabled!(log::Level::Debug) {
            debug!("Lyndon factorization : {:?}", factors);
        }
        assert_eq!(factors, core::isa_lyndon_factorization(&isa));
    }
}

#[test]
fn test_bwt_from_text_by_sa() {
    for i in 1..8 {
        //@ only for uneven (counting starts at one) Fibonacci words, we have the property that the BWT has exactly two runs. See https://dx.doi.org/10.1007/978-3-319-23660-5_12
        let text = fibonacci::fibonacci(2*i+1); 
        let bwt = core::bwt_from_text_by_sa(&text);
        let runs = core::number_of_runs(&mut bwt.as_slice());
        assert_eq!(runs, 2);
    }
}

#[test]
fn test_bwt_by_matrix() {
   pub const MAX_TEST_ITER : usize = 4096;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
        if text.len() < 2 { continue; }
        let naive  = core::bwt_by_matrix_naive(&text[0..text.len()-1]);
        let clever = core::bwt_by_matrix(&text[0..text.len()-1]);
        if naive != clever {
            core::bwt_by_matrix(&text[0..text.len()-1]);
        }
        assert_eq!(naive, clever);
    }
}

#[test]
fn test_lyndon_conjugate() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let n = text.len();
       let lconjugate = core::lyndon_conjugate(&text);
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


#[test]
fn test_border_array() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let border = core::border_array(&text);
       assert_eq!(border[0],0);
       assert_eq!(border[1],0);
       for i in 2..text.len() {
          for b in 0..border[i] {
             assert_eq!(text[b], text[i-border[i]+b]);
          }
       }
    }
}

#[test]
fn test_period() {
    const MAX_TEST_ITER : usize = 4096;
    // use crate::test;
    for text in core::RandomStringFactory::new(0..MAX_TEST_ITER as usize, 1) {
       let border = core::border_array(&text);
       let period = core::smallest_period(&border);
       for i in 0..text.len()-period {
          assert_eq!(text[i], text[i+period]);
       }
    }
}

