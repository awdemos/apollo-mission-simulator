use vstd::prelude::*;

verus! {

pub struct W {
    pub a: u8,
    pub b: u8,
}

spec fn ones8(d: u8) -> int {
    (d & 1u8) as int + ((d >> 1u8) & 1u8) as int + ((d >> 2u8) & 1u8) as int
        + ((d >> 3u8) & 1u8) as int + ((d >> 4u8) & 1u8) as int + ((d >> 5u8) & 1u8) as int
        + ((d >> 6u8) & 1u8) as int + ((d >> 7u8) & 1u8) as int
}

spec fn parity_spec(d: u8) -> u8 {
    if ones8(d) % 2 == 0 {
        1
    } else {
        2
    }
}

fn calc_parity(data: u8) -> (p: u8)
    ensures
        p == parity_spec(data),
{
    let b0 = data & 1u8;
    let b1 = (data >> 1u8) & 1u8;
    let b2 = (data >> 2u8) & 1u8;
    let b3 = (data >> 3u8) & 1u8;
    let b4 = (data >> 4u8) & 1u8;
    let b5 = (data >> 5u8) & 1u8;
    let b6 = (data >> 6u8) & 1u8;
    let b7 = (data >> 7u8) & 1u8;
    let count: u32 = b0 as u32 + b1 as u32 + b2 as u32 + b3 as u32 + b4 as u32 + b5 as u32
        + b6 as u32 + b7 as u32;
    proof {
        assert(count as int == ones8(data));
        assert(count % 2 == ones8(data) % 2);
    }
    if count % 2 == 0 {
        1
    } else {
        2
    }
}

proof fn lemma_u16_be(b0: u8, b1: u8)
    ensures
        (((b0 as u16) << 8u16) | (b1 as u16)) as int == b0 as int * 256 + b1 as int,
{
    assert((((b0 as u16) << 8u16) | (b1 as u16)) == ((b0 as u16) * 256u16 + b1 as u16)) by (bit_vector);
}

spec fn read_u16_int(s: Seq<u8>, i: int) -> int {
    s[i] as int * 256 + s[i + 1] as int
}

fn read_u16(s: &Vec<u8>, i: usize) -> (x: u16)
    requires
        i + 1 < s.len(),
    ensures
        x as int == read_u16_int(s@, i as int),
{
    let b0 = s[i];
    let b1 = s[i + 1];
    let x = ((b0 as u16) << 8u16) | (b1 as u16);
    proof {
        lemma_u16_be(b0, b1);
    }
    x
}

spec fn crc_prefix(s: Seq<u8>, n: int) -> int
    decreases n,
{
    if n <= 0 {
        0
    } else {
        crc_prefix(s, n - 1) + n * s[n - 1] as int
    }
}

spec fn crc_spec(s: Seq<u8>) -> int {
    crc_prefix(s, s.len() as int)
}

fn crc_compute(bytes: &Vec<u8>, end: usize) -> (c: u32)
    requires
        end <= bytes.len(),
        end <= 266,
    ensures
        c as int == crc_prefix(bytes@, end as int),
{
    let mut sum: u64 = 0;
    let mut i: usize = 0;
    while i < end
        invariant
            i <= end,
            sum as int == crc_prefix(bytes@, i as int),
            sum <= 10_000_000,
            i <= 266,
        decreases (end - i) as int,
    {
        let bi = bytes[i] as u64;
        proof {
            assert(bi <= 255);
            assert((i as u64 + 1) * bi <= 267 * 255) by (nonlinear_arith)
                requires
                    i <= 266,
                    bi <= 255,
            ;
            assert(sum + (i as u64 + 1) * bi <= 10_000_000 + 267 * 255) by (nonlinear_arith)
                requires
                    sum <= 10_000_000,
                    (i as u64 + 1) * bi <= 267 * 255,
            ;
        }
        sum = sum + (i as u64 + 1) * bi;
        i = i + 1;
    }
    proof {
        assert(sum <= 0xFFFF_FFFFu64) by (nonlinear_arith)
            requires
                sum <= 10_000_000,
        ;
    }
    sum as u32
}

proof fn lemma_crc_no_touch(s1: Seq<u8>, s2: Seq<u8>, n: int)
    requires
        0 <= n <= s1.len(),
        0 <= n <= s2.len(),
        forall|j: int| 0 <= j < n ==> s1[j] == s2[j],
    ensures
        crc_prefix(s1, n) == crc_prefix(s2, n),
    decreases n,
{
    if n > 0 {
        lemma_crc_no_touch(s1, s2, n - 1);
    }
}

proof fn lemma_crc_changed(s: Seq<u8>, k: int, v: u8, n: int)
    requires
        0 <= k < n,
        n <= s.len(),
        v != s[k],
    ensures
        crc_prefix(s.update(k, v), n) != crc_prefix(s, n),
    decreases n - k,
{
    if n > k + 1 {
        lemma_crc_changed(s, k, v, n - 1);
        let u = s.update(k, v);
        assert(u[n - 1] == s[n - 1]);
        assert(crc_prefix(u, n) == crc_prefix(u, n - 1) + n * u[n - 1] as int);
        assert(crc_prefix(s, n) == crc_prefix(s, n - 1) + n * s[n - 1] as int);
    } else {
        let u = s.update(k, v);
        assert(u[k] == v);
        assert(crc_prefix(u, k + 1) == crc_prefix(u, k) + (k + 1) * v as int);
        assert(crc_prefix(s, k + 1) == crc_prefix(s, k) + (k + 1) * s[k] as int);
        lemma_crc_no_touch(u, s, k);
        assert(crc_prefix(u, k) == crc_prefix(s, k));
        assert(crc_prefix(u, k + 1) - crc_prefix(s, k + 1) == (k + 1) * v as int - (k + 1) * s[k] as int);
        assert((k + 1) * v as int != (k + 1) * s[k] as int) by (nonlinear_arith)
            requires
                k + 1 >= 1,
                v as int != s[k] as int,
        ;
        assert(crc_prefix(u, k + 1) != crc_prefix(s, k + 1));
    }
}

pub const RATE_BPS_TABLE: [u64; 4] = [160, 1600, 51200, 0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRate {
    Emergency160bps,
    LowRate1_6kbps,
    HighRate51_2kbps,
    VoiceOnly,
}

spec fn rate_index_spec(r: DataRate) -> int {
    match r {
        DataRate::Emergency160bps => 0,
        DataRate::LowRate1_6kbps => 1,
        DataRate::HighRate51_2kbps => 2,
        DataRate::VoiceOnly => 3,
    }
}

fn rate_index(r: DataRate) -> (i: usize)
    ensures
        i < 4,
        i as int == rate_index_spec(r),
{
    match r {
        DataRate::Emergency160bps => 0,
        DataRate::LowRate1_6kbps => 1,
        DataRate::HighRate51_2kbps => 2,
        DataRate::VoiceOnly => 3,
    }
}

spec fn rate_bps_spec(r: DataRate) -> u64 {
    match r {
        DataRate::Emergency160bps => 160,
        DataRate::LowRate1_6kbps => 1600,
        DataRate::HighRate51_2kbps => 51200,
        DataRate::VoiceOnly => 0,
    }
}

fn bits_per_second(r: DataRate) -> (v: u64)
    ensures
        v == RATE_BPS_TABLE[rate_index_spec(r) as int],
        v == rate_bps_spec(r),
{
    RATE_BPS_TABLE[rate_index(r)]
}

fn set_smoke() -> (b: bool)
    ensures
        b,
{
    let mut v: Vec<u8> = Vec::new();
    v.push(1u8);
    v.push(2u8);
    v.set(0, 9u8);
    proof {
        assert(v@[0] == 9u8);
        assert(v@[1] == 2u8);
        assert(v@ == seq![9u8, 2]);
    }
    true
}

} // verus!

fn main() {}
