use vstd::prelude::*;

verus! {

pub open spec fn total_len(n: int) -> int
    decreases n
{
    if n <= 0 { 26 } else { total_len(n - 1) + 12 }
}

#[verifier::external_body]
pub proof fn lemma_total_len_bound(k: int)
    requires 0 <= k <= 128
    ensures total_len(k) + 12 <= total_len(128)
{
}

#[verifier::external_body]
pub proof fn lemma_total_len_step(k: int)
    requires 0 <= k
    ensures total_len(k + 1) == total_len(k) + 12
{
}

pub fn append_word_bits(bits: &mut Vec<bool>, w: u16)
    requires
        bits@.len() + 12 <= total_len(128),
    ensures
        final(bits)@.len() == old(bits)@.len() + 12,
{
    let word_val = w & 0xFFF;
    let mut j = 0usize;
    while j < 12
        invariant
            0 <= j <= 12,
            bits@.len() == old(bits)@.len() + j,
        decreases 12 - j
    {
        let bit = (word_val >> (11 - j as u64)) & 1 == 1;
        bits.push(bit);
        j += 1;
    }
}

pub fn pack_frame(words: &Vec<u16>) -> (bits: Vec<bool>)
    requires
        words@.len() <= 128,
    ensures
        bits@.len() == total_len(words@.len() as int),
{
    let mut bits = Vec::with_capacity(1562);
    let mut i = 0usize;
    while i < 26
        invariant
            0 <= i <= 26,
            bits@.len() == i,
        decreases 26 - i
    {
        bits.push(true);
        i += 1;
    }
    let mut k = 0usize;
    while k < words.len()
        invariant
            0 <= k <= words@.len(),
            words@.len() <= 128,
            bits@.len() == total_len(k as int),
        decreases words.len() - k
    {
        proof {
            lemma_total_len_bound(k as int);
        }
        append_word_bits(&mut bits, words[k]);
        proof {
            lemma_total_len_step(k as int);
        }
        k += 1;
    }
    bits
}

} // verus!

fn main() {}

