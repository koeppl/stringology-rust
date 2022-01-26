#[allow(dead_code)] extern crate env_logger;
#[macro_use] extern crate more_asserts;
#[allow(dead_code)] mod io;
#[allow(dead_code)] mod core;

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
