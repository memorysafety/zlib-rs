#![no_main]

//! the tests provide good coverage, the purose of this fuzzer is to
//! discover memory safety issues in the SIMD implementations.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (Vec<u8>, u32)| {
    let (input, start) = input;

    {
        let expected = {
            let mut h = crc32fast::Hasher::new_with_initial(start);
            h.update(&input[..]);
            h.finalize()
        };

        let actual = zlib_rs::crc32::crc32(input.as_slice(), start);

        assert_eq!(expected, actual);
    }

    {
        let expected = {
            let mut h = crc32fast::Hasher::new_with_initial(0);
            h.update(&input[..]);
            h.finalize()
        };

        let mut buf = [0; 1 << 16];
        let mut dst = zlib_rs::read_buf::ReadBuf::new(&mut buf[..input.len()]);

        let actual = zlib_rs::crc32::crc32_copy(&mut dst, input.as_slice());

        assert_eq!(expected, actual);

        assert_eq!(input, dst.filled());
    }
});
