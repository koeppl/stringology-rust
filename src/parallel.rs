#[allow(dead_code)] mod io;
#[allow(dead_code)] mod core;
#[macro_use] extern crate more_asserts;
use std::thread;

extern crate no_deadlocks;


// test cases: aaaaaaa -> 012345 ...
// ababab -> 0022345...




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



   // let counter = Arc::new(Mutex::new([0;max_length]));
   // let countr2 = Arc::new(Mutex::new(Vec::new()));
   // // let mut c = countr2.lock().unwrap();
   // for _ in 0..max_length {
   //    // countr2.lock().unwrap().push(Vec::<u8>::new());
   //    countr2.lock().unwrap().push(Arc::new(Mutex::new(Vec::<u8>::new())));
   // }
   // let mut handles = vec![];
   //
   // for _ in 0..10 {
   //    let counter = Arc::clone(&counter);
   //    let handle = thread::spawn({
   //       let clone = Arc::clone(&countr2);
   //       move || {
   //       let mut num = counter.lock().unwrap();
   //       let c = clone.lock().unwrap();
   //       let mut vec = c[3].lock().unwrap();
   //
   //       let bbwt = vec![0u8; 7];
   //       vec.extend_from_slice(&bbwt);
   //       num[5] += 1;
   //        println!("fact = {}", num[5]);
   //       }
   //    });
   //    handles.push(handle);
   // }

   // for handle in handles {
   //    handle.join().unwrap();
   // }

fn number_distinct_lyndon_factors(text: &[u8]) -> usize {
   if text.len() == 0 { return 0; }
   let conjugates = core::duval(&text);
   if conjugates.len() == 1 { return 1; }
   
   let mut counter = 1 as usize;
   if conjugates[1]-conjugates[0] != 1+conjugates[0] { //@ different factor lengths means different factor
      counter += 1;
   } else {
      for i in 0..conjugates[0] {
         if text[i] != text[conjugates[0]+1+i] {
            counter += 1;
            break;
         }
      }
   }
   for it in 1..conjugates.len()-1 {
      if conjugates[it]-conjugates[it-1] != conjugates[it+1]-conjugates[it] { //@ different factor lengths means different factor
         counter += 1;
      } else {
         for i in 0..conjugates[it]-conjugates[it-1] {
            if text[conjugates[it-1]+i+1] != text[conjugates[it]+i+1] {
               counter += 1;
               break;
            }
         }
      }
   }
   counter
}

#[test]
fn test_number_distinct_lyndon_factors() {
   assert_eq!(1, number_distinct_lyndon_factors(b"a"));
   assert_eq!(1, number_distinct_lyndon_factors(b"aa"));
   assert_eq!(1, number_distinct_lyndon_factors(b"aaaaaaaaa"));
   assert_eq!(2, number_distinct_lyndon_factors(b"ababa"));
   assert_eq!(2, number_distinct_lyndon_factors(b"abababaaaa"));
   assert_eq!(2, number_distinct_lyndon_factors(b"abababa"));
   assert_eq!(3, number_distinct_lyndon_factors(b"abracadabra"));
   assert_eq!(3, number_distinct_lyndon_factors(b"ababbababbabababaaaa"));
   assert_eq!(4, number_distinct_lyndon_factors(b"ababbababbbababbababbabababaaaa"));
}


fn main() {
   const max_length : usize = 24;
   use std::sync::{Arc};
   use no_deadlocks::{Mutex,RwLock};
   // use std::sync::{Mutex,RwLock};


   for pattern_length in 16..max_length {
      let stats_arc = Arc::new(Mutex::new([[0;max_length+1];max_length+1]));
      let iterator_arc = Arc::new(Mutex::new(core::LyndonWordGenerator::new(max_length, 2)));
      let total_counter_arc = Arc::new(RwLock::new(0 as usize));
      let mut handles = vec![];
      for _ in 0..4 {
         let total_counter_arc = Arc::clone(&total_counter_arc);
         let stats_arc = Arc::clone(&stats_arc);
         let iterator_arc = Arc::clone(&iterator_arc);
         let handle = thread::spawn(move || {
            loop {
               let m = {
                  let mut iterator = iterator_arc.lock().unwrap();
                  iterator.next()
               };
               match m {
                  None => break, 
                  Some(text) => if text.len() == pattern_length {
                     {
                        // println!("fact = {:?}", text);
                        let bwt = core::bwt_by_matrix_naive(&text);
                        let bwt_runs = core::number_of_runs(&mut bwt.as_slice());

                        let mut best_bbwt_runs = max_length;
                        assert_gt!(text.len(), 0);
                        let lindex = text.len()-1;
                        assert_lt!(lindex, max_length);
                        for conjugate in core::ConjugateIterator::new(&text) {
                           let bbwt = core::bbwt_naive(&conjugate);
                           let bbwt_runs = core::number_of_runs(&mut bbwt.as_slice());
                           let num_conjugates = number_distinct_lyndon_factors(&conjugate);
                           if bbwt_runs < num_conjugates  {
                              println!("bbwt={} conjugate={} bbwt_runs={} #factors={}", binary_vector_to_str(&bbwt), binary_vector_to_str(&conjugate), bbwt_runs, num_conjugates);
                           }

                           if best_bbwt_runs > bbwt_runs {
                              best_bbwt_runs = bbwt_runs;
                           }
                        }

                        {
                           let mut stats = stats_arc.lock().unwrap();
                           stats[bwt_runs][best_bbwt_runs] += 1;
                        }

                        {
                           let mut total_counter = total_counter_arc.write().unwrap();
                           *total_counter += 1;
                        }
                     }
                  }
               }
            }
         });
         handles.push(handle);
      }
      for handle in handles {
         handle.join().unwrap();
      }

      let my_total_counter = {
         let total_counter = total_counter_arc.read().unwrap();
         *total_counter
      };
      // if my_total_counter % 10 == 0 {
      {
         let stats = stats_arc.lock().unwrap();

         let mut max_val = 0;
         for br in 0..max_length {
            for bbr in 0..max_length {
               if max_val < stats[br][bbr] {
                  max_val = stats[br][bbr];
               }
            }
         }
         let width=std::cmp::max(2, max_val.to_string().len());
         println!("binary string length: {} ", pattern_length);
         println!("bwt \\ bbwt : #= {}", my_total_counter);

         for br in 0..max_length {
            print!("{number:>width$}|", number=br, width=width);
         }
         println!("");
         for br in 1..max_length {
            print!("{number:>width$}|", number=br,width=width);
            for bbr in 1..max_length {
               print!("{number:>width$}|", number=stats[br][bbr],width=width);
            }
            println!("");
         }
      }


   }
}

fn probe_main() {
   const max_length : usize = 16;
   let it = core::LyndonWordGenerator::new(max_length, 2);
   for text in it {
      // println!("fact = {:?}", text);
      let bwt = core::bwt_by_matrix_naive(&text);
      let bwt_runs = core::number_of_runs(&mut bwt.as_slice());

      let mut bbwt_wins = 0;
      let mut best_conjugate = text.clone();

      let mut best_bbwt_runs = max_length;
      assert_gt!(text.len(), 0);
      let lindex = text.len()-1;
      assert_lt!(lindex, max_length);
      for conjugate in core::ConjugateIterator::new(&text) {
         let bbwt = core::bbwt_naive(&conjugate);
         let bbwt_runs = core::number_of_runs(&mut bbwt.as_slice());
         if bbwt_runs < bwt_runs {
            bbwt_wins += 1;
         }
         if best_bbwt_runs > bbwt_runs {
            best_bbwt_runs = bbwt_runs;
            best_conjugate = conjugate.clone();
         }
      }
      //if bbwt_wins == 1 && best_bbwt_runs+2 < bwt_runs {
      if bbwt_wins == 0 && bwt_runs >= 10  {
         println!("text={} conj={} rbwt={} rbbwt={}", binary_vector_to_str(&text), binary_vector_to_str(&best_conjugate), bwt_runs, best_bbwt_runs);

      }
   }
}

fn max_lyndon_word() {
   const max_length : usize = 40;
   use std::sync::{Arc, Mutex};
   use std::sync::RwLock;


   let num_lyndon_words_arc  = Arc::new(Mutex::new([0;max_length]));
   let bwt_win_counter_arc   = Arc::new(Mutex::new([0;max_length])); 
   let bbwt_win_counter_arc  = Arc::new(Mutex::new([0;max_length])); 
   let tie_win_counter_arc   = Arc::new(Mutex::new([0;max_length])); 
   let best_bbwt_bbwtrun_arc = Arc::new(RwLock::new([0;max_length]));
   let best_bbwt_bwtrun_arc  = Arc::new(RwLock::new([0;max_length]));


   let best_bbwt_text_arc = Arc::new(RwLock::new(Vec::new()));
   for _ in 0..max_length {
      best_bbwt_text_arc.write().unwrap().push(Arc::new(RwLock::new(Vec::<u8>::new())));
   }
   let mut handles = vec![];

   // let mut score_counter = 0i64; // same, but stores the diff
   // let mut best_bbwt_text : [Vec<u8>; max_length] = Default::default();
   // let mut best_bbwt_text = std::iter::repeat(vec![]).take(max_length).collect::<Vec<_>>();


   let total_counter_arc = Arc::new(Mutex::new(0 as usize));

   let iterator_arc = Arc::new(Mutex::new(core::LyndonWordGenerator::new(max_length, 2)));

   for _ in 0..4 {
      let num_lyndon_words_arc  = Arc::clone(&num_lyndon_words_arc );
      let bwt_win_counter_arc   = Arc::clone(&bwt_win_counter_arc   );
      let bbwt_win_counter_arc  = Arc::clone(&bbwt_win_counter_arc  );
      let tie_win_counter_arc   = Arc::clone(&tie_win_counter_arc   );
      let best_bbwt_bbwtrun_arc = Arc::clone(&best_bbwt_bbwtrun_arc );
      let best_bbwt_bwtrun_arc  = Arc::clone(&best_bbwt_bwtrun_arc  );
      let best_bbwt_text_arc    = Arc::clone(&best_bbwt_text_arc   );
      let iterator_arc = Arc::clone(&iterator_arc);
      let total_counter_arc = Arc::clone(&total_counter_arc);
      let handle = thread::spawn(move || {
         loop {
            let m = {
               let mut iterator = iterator_arc.lock().unwrap();
               iterator.next()
            };
            match m {
               None => break, 
               Some(text) => {
                  {
                     // println!("fact = {:?}", text);
                     let bwt = core::bwt_by_matrix_naive(&text);
                     let bwt_runs = core::number_of_runs(&mut bwt.as_slice());

                     assert_gt!(text.len(), 0);
                     let lindex = text.len()-1;
                     assert_lt!(lindex, max_length);
                     for conjugate in core::ConjugateIterator::new(&text) {
                        // println!("conj = {:?}", conjugate);
                        // assert_eq!(bwt, bwt_by_matrix_naive(&conjugate));  
                        let bbwt = core::bbwt_naive(&conjugate);
                        let bbwt_runs = core::number_of_runs(&mut bbwt.as_slice());

                        {
                           let mut bwt_win_counter = bwt_win_counter_arc.lock().unwrap();
                           let mut bbwt_win_counter = bbwt_win_counter_arc.lock().unwrap();
                           let mut tie_win_counter = tie_win_counter_arc.lock().unwrap();

                           if bwt_runs < bbwt_runs { bwt_win_counter[lindex] += 1; } else if bwt_runs > bbwt_runs { bbwt_win_counter[lindex] += 1 } else { tie_win_counter[lindex] += 1 };
                        }

                        if bbwt_runs < bwt_runs {
                           let mut needs_change = false;

                           {
                              let best_bbwt_bwtrun = best_bbwt_bwtrun_arc.read().unwrap();
                              let best_bbwt_bbwtrun = best_bbwt_bbwtrun_arc.read().unwrap();
                              let best_bbwt_text = best_bbwt_text_arc.read().unwrap();
                              let texthandle = best_bbwt_text[lindex].read().unwrap();

                              if bwt_runs-bbwt_runs > (best_bbwt_bwtrun[lindex]-best_bbwt_bbwtrun[lindex]) ||
                                 (bwt_runs-bbwt_runs == (best_bbwt_bwtrun[lindex]-best_bbwt_bbwtrun[lindex]) && second_criteria(&conjugate, & texthandle.as_slice())) {
                                    needs_change = true;
                              }
                           }
                           if needs_change  {
                                 let mut best_bbwt_bwtrun = best_bbwt_bwtrun_arc.write().unwrap();
                                 let mut best_bbwt_bbwtrun = best_bbwt_bbwtrun_arc.write().unwrap();
                                 let best_bbwt_text = best_bbwt_text_arc.write().unwrap();
                                 let mut texthandle = best_bbwt_text[lindex].write().unwrap();

                                 // let mut best_bbwt_bbwtrun = best_bbwt_bbwtrun_arc.lock().unwrap();
                                 best_bbwt_bwtrun[lindex] = bwt_runs;
                                 best_bbwt_bbwtrun[lindex] = bbwt_runs;
                                 texthandle.clear();
                                 texthandle.extend_from_slice(&conjugate);
                           }
                        }

                     }
                     let mut num_lyndon_words = num_lyndon_words_arc.lock().unwrap();
                     num_lyndon_words[lindex] += 1;

                     let mut total_counter = total_counter_arc.lock().unwrap();
                     *total_counter += 1;
                  }
                  let total_counter = total_counter_arc.lock().unwrap();
                  if *total_counter % 10000 == 0 {
                     let num_lyndon_words = num_lyndon_words_arc.lock().unwrap();
                     let bwt_win_counter = bwt_win_counter_arc.lock().unwrap();
                     let bbwt_win_counter = bbwt_win_counter_arc.lock().unwrap();
                     let tie_win_counter = tie_win_counter_arc.lock().unwrap();
                     let best_bbwt_bwtrun = best_bbwt_bwtrun_arc.read().unwrap();
                     let best_bbwt_bbwtrun = best_bbwt_bbwtrun_arc.read().unwrap();
                     let best_bbwt_text = best_bbwt_text_arc.read().unwrap();
                     println!("total_count={}", *total_counter);
                     for length in 20..max_length {
                        let texthandle = best_bbwt_text[length].read().unwrap();
                        println!("length={} lyndon_words={} bwt_wins={} bbwt_wins={} ties={}", length, num_lyndon_words[length], bwt_win_counter[length], bbwt_win_counter[length], tie_win_counter[length]);
                        // println!("length={} text={} bwt_runs={} bbwt_runs={}", length, str::from_utf8(&best_bbwt_text[length]).unwrap(), best_bbwt_bwtrun[length], best_bbwt_bbwtrun[length]);
                        println!("length={} text={} bwt_runs={} bbwt_runs={}", length, binary_vector_to_str(&texthandle), best_bbwt_bwtrun[length], best_bbwt_bbwtrun[length]);
                     }
                  }
               }
            }
         }
      });
      handles.push(handle);
   }
   for handle in handles {
      handle.join().unwrap();
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
    //     let bwt = bwt_by_matrix_naive(&text);
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
    //         let bwt = bwt_by_matrix_naive(&text);
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
