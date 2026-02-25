//! SIMD-accelerated view application.
//!
//! Uses shuffle instructions to process multiple bytes per instruction.
//! The 256-entry table lookup is implemented using a 16-way masked shuffle
//! approach with `vpshufb` (AVX2) or `pshufb` (SSE4.2).

/// AVX2-accelerated 256-entry table lookup.
///
/// Uses a 16-way masked shuffle approach:
/// 1. For each high nibble value (0-15), load the corresponding 16-byte subtable
/// 2. Mask bytes where high nibble matches
/// 3. Use vpshufb to lookup using low nibble as index
/// 4. Accumulate results
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub(super) fn apply_avx2(view: &crate::view::ElementWiseView, data: &mut [u8]) {
    use core::arch::x86_64::*;

    let table = view.table();
    let len = data.len();
    let chunks = len / 32;

    unsafe {
        // Constant for extracting high nibble: 0x0F0F0F0F...
        let low_nibble_mask = _mm256_set1_epi8(0x0F);

        // Process 32 bytes at a time
        for i in 0..chunks {
            let offset = i * 32;
            let ptr = data.as_mut_ptr().add(offset);

            // Load 32 input bytes
            let input = _mm256_loadu_si256(ptr as *const __m256i);

            // Extract low nibbles (indices into 16-byte subtables)
            let low_nibbles = _mm256_and_si256(input, low_nibble_mask);

            // Extract high nibbles shifted right by 4
            let high_nibbles = _mm256_and_si256(_mm256_srli_epi16(input, 4), low_nibble_mask);

            // Accumulator for results
            let mut result = _mm256_setzero_si256();

            // Process each subtable (high nibble values 0-15)
            for h in 0..16u8 {
                // Load 16-byte subtable, broadcast to both 128-bit lanes
                let subtable_offset = (h as usize) * 16;
                let subtable_lo =
                    _mm_loadu_si128(table.as_ptr().add(subtable_offset) as *const __m128i);
                let subtable = _mm256_broadcastsi128_si256(subtable_lo);

                // Create mask for bytes where high nibble == h
                let h_vec = _mm256_set1_epi8(h as i8);
                let mask = _mm256_cmpeq_epi8(high_nibbles, h_vec);

                // Use vpshufb to lookup: result = subtable[low_nibble] for each byte
                let lookup = _mm256_shuffle_epi8(subtable, low_nibbles);

                // Select only bytes that match this high nibble
                let masked_lookup = _mm256_and_si256(lookup, mask);

                // OR into accumulator
                result = _mm256_or_si256(result, masked_lookup);
            }

            // Store result
            _mm256_storeu_si256(ptr as *mut __m256i, result);
        }

        // Handle remainder with scalar path
        for i in (chunks * 32)..len {
            data[i] = table[data[i] as usize];
        }
    }
}

/// AVX2-accelerated 256-entry table lookup with separate input/output.
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub(super) fn apply_to_avx2(view: &crate::view::ElementWiseView, input: &[u8], output: &mut [u8]) {
    use core::arch::x86_64::*;

    let table = view.table();
    let len = input.len();
    let chunks = len / 32;

    unsafe {
        let low_nibble_mask = _mm256_set1_epi8(0x0F);

        for i in 0..chunks {
            let offset = i * 32;
            let in_ptr = input.as_ptr().add(offset);
            let out_ptr = output.as_mut_ptr().add(offset);

            let input_vec = _mm256_loadu_si256(in_ptr as *const __m256i);
            let low_nibbles = _mm256_and_si256(input_vec, low_nibble_mask);
            let high_nibbles = _mm256_and_si256(_mm256_srli_epi16(input_vec, 4), low_nibble_mask);

            let mut result = _mm256_setzero_si256();

            for h in 0..16u8 {
                let subtable_offset = (h as usize) * 16;
                let subtable_lo =
                    _mm_loadu_si128(table.as_ptr().add(subtable_offset) as *const __m128i);
                let subtable = _mm256_broadcastsi128_si256(subtable_lo);

                let h_vec = _mm256_set1_epi8(h as i8);
                let mask = _mm256_cmpeq_epi8(high_nibbles, h_vec);
                let lookup = _mm256_shuffle_epi8(subtable, low_nibbles);
                let masked_lookup = _mm256_and_si256(lookup, mask);
                result = _mm256_or_si256(result, masked_lookup);
            }

            _mm256_storeu_si256(out_ptr as *mut __m256i, result);
        }

        // Handle remainder
        for i in (chunks * 32)..len {
            output[i] = table[input[i] as usize];
        }
    }
}

/// SSE4.2-accelerated 256-entry table lookup.
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
pub(super) fn apply_sse42(view: &crate::view::ElementWiseView, data: &mut [u8]) {
    use core::arch::x86_64::*;

    let table = view.table();
    let len = data.len();
    let chunks = len / 16;

    unsafe {
        let low_nibble_mask = _mm_set1_epi8(0x0F);

        for i in 0..chunks {
            let offset = i * 16;
            let ptr = data.as_mut_ptr().add(offset);

            let input = _mm_loadu_si128(ptr as *const __m128i);
            let low_nibbles = _mm_and_si128(input, low_nibble_mask);
            let high_nibbles = _mm_and_si128(_mm_srli_epi16(input, 4), low_nibble_mask);

            let mut result = _mm_setzero_si128();

            for h in 0..16u8 {
                let subtable_offset = (h as usize) * 16;
                let subtable =
                    _mm_loadu_si128(table.as_ptr().add(subtable_offset) as *const __m128i);

                let h_vec = _mm_set1_epi8(h as i8);
                let mask = _mm_cmpeq_epi8(high_nibbles, h_vec);
                let lookup = _mm_shuffle_epi8(subtable, low_nibbles);
                let masked_lookup = _mm_and_si128(lookup, mask);
                result = _mm_or_si128(result, masked_lookup);
            }

            _mm_storeu_si128(ptr as *mut __m128i, result);
        }

        // Handle remainder
        for i in (chunks * 16)..len {
            data[i] = table[data[i] as usize];
        }
    }
}

/// SSE4.2-accelerated 256-entry table lookup with separate input/output.
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
pub(super) fn apply_to_sse42(view: &crate::view::ElementWiseView, input: &[u8], output: &mut [u8]) {
    use core::arch::x86_64::*;

    let table = view.table();
    let len = input.len();
    let chunks = len / 16;

    unsafe {
        let low_nibble_mask = _mm_set1_epi8(0x0F);

        for i in 0..chunks {
            let offset = i * 16;
            let in_ptr = input.as_ptr().add(offset);
            let out_ptr = output.as_mut_ptr().add(offset);

            let input_vec = _mm_loadu_si128(in_ptr as *const __m128i);
            let low_nibbles = _mm_and_si128(input_vec, low_nibble_mask);
            let high_nibbles = _mm_and_si128(_mm_srli_epi16(input_vec, 4), low_nibble_mask);

            let mut result = _mm_setzero_si128();

            for h in 0..16u8 {
                let subtable_offset = (h as usize) * 16;
                let subtable =
                    _mm_loadu_si128(table.as_ptr().add(subtable_offset) as *const __m128i);

                let h_vec = _mm_set1_epi8(h as i8);
                let mask = _mm_cmpeq_epi8(high_nibbles, h_vec);
                let lookup = _mm_shuffle_epi8(subtable, low_nibbles);
                let masked_lookup = _mm_and_si128(lookup, mask);
                result = _mm_or_si128(result, masked_lookup);
            }

            _mm_storeu_si128(out_ptr as *mut __m128i, result);
        }

        // Handle remainder
        for i in (chunks * 16)..len {
            output[i] = table[input[i] as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "x86_64")]
    use crate::view::ElementWiseView;

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    use super::{apply_avx2, apply_to_avx2};

    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    use super::{apply_sse42, apply_to_sse42};

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_avx2_identity() {
        let view = ElementWiseView::identity();
        let mut data: Vec<u8> = (0..64).collect();
        let expected: Vec<u8> = (0..64).collect();
        apply_avx2(&view, &mut data);
        assert_eq!(data, expected);
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_avx2_increment() {
        let view = ElementWiseView::new(|x| x.wrapping_add(1));
        let mut data: Vec<u8> = (0..64).collect();
        apply_avx2(&view, &mut data);
        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(byte, ((i + 1) % 256) as u8);
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_avx2_xor() {
        let view = ElementWiseView::new(|x| x ^ 0xFF);
        let mut data: Vec<u8> = (0..64).collect();
        apply_avx2(&view, &mut data);
        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(byte, (i as u8) ^ 0xFF);
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_avx2_all_values() {
        // Test all 256 input values
        let view = ElementWiseView::new(|x| x.wrapping_mul(3).wrapping_add(7));
        let mut data: Vec<u8> = (0..=255).collect();
        // Extend to multiple of 32
        data.extend(0..=255);
        let expected: Vec<u8> = data
            .iter()
            .map(|&x| x.wrapping_mul(3).wrapping_add(7))
            .collect();
        apply_avx2(&view, &mut data);
        assert_eq!(data, expected);
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_to_avx2() {
        let view = ElementWiseView::new(|x| x.wrapping_add(10));
        let input: Vec<u8> = (0..64).collect();
        let mut output = vec![0u8; 64];
        apply_to_avx2(&view, &input, &mut output);
        for (i, &byte) in output.iter().enumerate() {
            assert_eq!(byte, ((i + 10) % 256) as u8);
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn test_apply_sse42_identity() {
        let view = ElementWiseView::identity();
        let mut data: Vec<u8> = (0..32).collect();
        let expected: Vec<u8> = (0..32).collect();
        apply_sse42(&view, &mut data);
        assert_eq!(data, expected);
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn test_apply_sse42_increment() {
        let view = ElementWiseView::new(|x| x.wrapping_add(1));
        let mut data: Vec<u8> = (0..32).collect();
        apply_sse42(&view, &mut data);
        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(byte, ((i + 1) % 256) as u8);
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn test_apply_to_sse42() {
        let view = ElementWiseView::new(|x| x ^ 0xAA);
        let input: Vec<u8> = (0..32).collect();
        let mut output = vec![0u8; 32];
        apply_to_sse42(&view, &input, &mut output);
        for (i, &byte) in output.iter().enumerate() {
            assert_eq!(byte, (i as u8) ^ 0xAA);
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    fn test_apply_avx2_with_remainder() {
        let view = ElementWiseView::new(|x| x.wrapping_add(5));
        let mut data: Vec<u8> = (0..50).collect(); // 32 + 18 remainder
        let expected: Vec<u8> = data.iter().map(|&x| x.wrapping_add(5)).collect();
        apply_avx2(&view, &mut data);
        assert_eq!(data, expected);
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn test_apply_sse42_with_remainder() {
        let view = ElementWiseView::new(|x| x.wrapping_add(5));
        let mut data: Vec<u8> = (0..25).collect(); // 16 + 9 remainder
        let expected: Vec<u8> = data.iter().map(|&x| x.wrapping_add(5)).collect();
        apply_sse42(&view, &mut data);
        assert_eq!(data, expected);
    }
}
