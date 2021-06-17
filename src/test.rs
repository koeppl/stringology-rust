use crate::datastructures;

pub struct StringTestFactory {
    m_range : std::ops::Range<usize>,
    m_log_alphabet_size : u8,
}

impl StringTestFactory {
    #[allow(dead_code)]
    pub fn new(r : std::ops::Range<usize>, log_alphabet_size : u8) -> StringTestFactory {
        StringTestFactory { m_range : r, m_log_alphabet_size : log_alphabet_size}
    }
}


impl Iterator for StringTestFactory {
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
            let most_significant_bit = datastructures::bit_size(iter_round);

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


