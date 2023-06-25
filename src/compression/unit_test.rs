#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_compress() {
        let bits = BitVec::from_bitslice(bits![0, 0, 1, 1]);
        let len = bits.len();
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!((vec![2, 2], len), compressed_bits);

        let bits = BitVec::from_bitslice(bits![1, 0, 1, 1]);
        let len = bits.len();
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!((vec![0, 1, 1, 2], len), compressed_bits);

        let bits = BitVec::from_bitslice(bits![1, 0, 1, 0]);
        let len = bits.len();
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!((vec![0, 1, 1, 1, 1], len), compressed_bits);
    }

    #[test]
    fn test_expand() {
        let bits = BitVec::from_bitslice(bits![0, 0, 1, 1]);
        let run_length = RunLength::init(bits);
        let (compressed_bits, init_len) = run_length.compress();
        let expanded_bits = run_length.expand(compressed_bits, 4);
        assert_eq!(&expanded_bits, run_length.bits());
    }

    #[test]
    fn test_mutate() {
        let mut run_length = RunLength::<u8, Lsb0>::new();
        run_length.push(true);
        run_length.push(false);
        run_length.extend_from_raw_slice(&[0, 1]);
        let bits = bits![1_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
        let bits = BitVec::<_, Lsb0>::from_bitslice(bits);
        assert_eq!(run_length.bits(), &bits);
    }
}
