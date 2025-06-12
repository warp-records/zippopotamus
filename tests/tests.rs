

use zippopotamus::lz77::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = [];
        let output = lz77_encode(&input);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_single_byte() {
        let input = [42];
        let output = lz77_encode(&input);
        // Expected: (len=0, offset=0, next_symbol=42)
        assert_eq!(output, vec![0, 0, 42]);
    }

    #[test]
    fn test_repeated_pattern() {
        let input = [1, 1, 1, 1];
        let output = lz77_encode(&input);
        // Expected:
        // - First byte: (len=0, offset=0, next_symbol=1)
        // - Subsequent bytes: (len=1, offset=1, next_symbol=1)
        assert_eq!(output, vec![0, 0, 1, 3, 1, 1]);
    }

    #[test]
    fn test_no_matches() {
        let input = [1, 2, 3, 4];
        let output = lz77_encode(&input);
        // Expected: Each byte is new (len=0, offset=0, next_symbol=current_byte)
        assert_eq!(output, vec![0, 0, 1, 0, 0, 2, 0, 0, 3, 0, 0, 4]);
    }

    #[test]
    fn test_partial_match() {
        let input = [1, 2, 1, 2, 3];
        let output = lz77_encode(&input);
        // Expected:
        // - First byte: (len=0, offset=0, next_symbol=1)
        // - Second byte: (len=0, offset=0, next_symbol=2)
        // - Third byte: (len=2, offset=2, next_symbol=3)
        assert_eq!(output, vec![0, 0, 1, 0, 0, 2, 2, 2, 3]);
    }
/*
    #[test]
    fn test_window_size_limit() {
        let input = [1; WINDOW_LEN + 1];
        let output = lz77_encode(&input);
        // Expected:
        // - First byte: (len=0, offset=0, next_symbol=1)
        // - Subsequent bytes: (len=1, offset=1, next_symbol=1)
        assert_eq!(output.len(), 3 * (WINDOW_LEN + 1));
    }
 */
    #[test]
    fn test_long_match() {
        let input = [1, 2, 3, 1, 2, 3, 4];
        let output = lz77_encode(&input);
        // Expected:
        // - First 3 bytes: (len=0, offset=0, next_symbol=current_byte)
        // - Fourth byte: (len=3, offset=3, next_symbol=4)
        assert_eq!(output, vec![0, 0, 1, 0, 0, 2, 0, 0, 3, 3, 3, 4]);
    }

    #[test]
    fn test_decode_empty_input() {
        let input = vec![];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_decode_single_byte() {
        let input = vec![0, 0, 42];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![42]);
    }


    #[test]
    fn test_decode_repeated_pattern() {
        let input = vec![0, 0, 1, 2, 1, 1];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_decode_no_matches() {
        let input = vec![0, 0, 1, 0, 0, 2, 0, 0, 3, 0, 0, 4];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_decode_partial_match() {
        let input = vec![0, 0, 1, 0, 0, 2, 2, 2, 3];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![1, 2, 1, 2, 3]);
    }

    #[test]
    fn test_decode_long_match() {
        let input = vec![0, 0, 1, 0, 0, 2, 0, 0, 3, 3, 3, 4];
        let output = lz77_decode(&input).unwrap();
        assert_eq!(output, vec![1, 2, 3, 1, 2, 3, 4]);
    }
}
