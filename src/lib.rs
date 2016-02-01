#![feature(test)]

#[cfg(test)] extern crate test;

pub fn faster_radix_sort(data: &mut [i32]) {
    // Let `count[n*256 + k]` be the number of elements in `data` for which the nth byte is k.
    let mut count = vec![0usize; 4 * 256];
    for r in data as &[i32] {
        let val = *r;
        count[                 (val & 0xFF) as usize] += 1;
        count[  256 +  ((val >>  8) & 0xFF) as usize] += 1;
        count[2*256 +  ((val >> 16) & 0xFF) as usize] += 1;
        count[3*256 + (((val >> 24) & 0xFF) ^ 0x80) as usize] += 1;
    }

    // Make a temporary buffer the same size as `data`.  We'll copy the
    // data back and forth between `data` and `tmpbuf`.
    let mut tmpbuf = vec![0i32; data.len()];

    let mut srcbuf: &mut [i32] = data;
    let mut dstbuf: &mut [i32] = &mut tmpbuf[..];

    for n in 0..4 {
        // Let `counts[k]` be the number of elements in srcbuf for which the nth
        // byte is `k`. (We already computed this above.)
        let counts = &mut count[256 * n .. 256 * (n + 1)];

        // Let `counts[i]` be the number of elements in srcbuf for which the nth
        // byte is **less than** `k`.
        let mut total = 0;
        for i in 0..256 {
            let n = counts[i];
            counts[i] = total;
            total += n;
        }
        assert_eq!(total, srcbuf.len());

        // At this point, `counts` contains an increasing sequence of indexes
        // into `dstbuf`.
        //
        //        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // dstbuf | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
        //        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        //         ^   ^ ^           ^           ^   ^     ^ ^     ^
        //         |   | |           |           |   |     | |     |
        // counts  0   1 2           3           4   5     6 7     8
        //
        // Note that the distance between counts[3] and counts[4] is exactly
        // the number of elements for which byte `n` is 3, and so on.
        //
        // Now it's like sorting mail in an old-school post office; but each
        // of our 256 mail slots is tailor-made to hold exactly the number of
        // items we need to put there. For each value in `srcbuf`, look at byte
        // `n` and copy the value into the appropriate "mail slot" in `dstbuf`.
        // `counts` tells us where that mail slot is.
        //
        for r in srcbuf as &[i32] {
            let val = *r;
            let mut k = (val >> (8 * n)) & 0xFFi32;
            if n == 3 {
                k ^= 0x80;
            }
            let i = counts[k as usize];
            dstbuf[i] = val;

            // Bump counts[k] to point to the next remaining empty space in
            // this "slot" (or one past the end of the slot, if it's full).
            counts[k as usize] = i + 1;
        }

        // Now just switch buffers. (This scheme only works because we do an
        // even number of passes!)
        std::mem::swap(&mut srcbuf, &mut dstbuf);
    }
}


pub fn radix_sort(data: &mut [i32]) {
    let mut buckets = vec![vec![]; 256];

    for n in 0..4 {
        for r in data as &[i32] {
            let val = *r;
            let mut which = (val >> (8 * n)) & 0xFFi32;
            if n == 3 {
                which ^= 0x80;
            }
            buckets[which as usize].push(val);
        }

        let mut i = 0;
        for b in buckets.iter_mut() {
            for r in b as &[i32] {
                data[i] = *r;
                i += 1;
            }
            b.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::radix_sort;
    use super::faster_radix_sort;

    #[test]
    fn it_works() {
        let mut data = vec![512, 4, 3, -1, 256, 1, 6];
        radix_sort(&mut data);
        assert_eq!(data, vec![-1, 1, 3, 4, 6, 256, 512]);
    }

    #[test]
    fn the_other_way_works() {
        let mut data = vec![512, 4, 3, -1, 256, 1, 6];
        faster_radix_sort(&mut data);
        assert_eq!(data, vec![-1, 1, 3, 4, 6, 256, 512]);
    }

    // This function returns the same vector of random-looking values each time
    // it's called.
    fn generate_unsorted(len: usize) -> Vec<i32> {
        let mut v = vec![0; len];

        // Fill v with the same numbers each time this benchmark is run.
        let mut x: u64 = 17;
        for i in 0 .. v.len() {
            v[i] = x.wrapping_mul(2685821657736338717) as i32;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
        }

        v
    }

    fn bench_radix_sort(b: &mut Bencher, len: usize) {
        let v = generate_unsorted(len);
        b.iter(|| {
            let mut my_v = v.clone();
            radix_sort(&mut my_v);
        });
    }

    fn bench_faster_radix_sort(b: &mut Bencher, len: usize) {
        let v = generate_unsorted(len);
        b.iter(|| {
            let mut my_v = v.clone();
            faster_radix_sort(&mut my_v);
        });
    }

    fn bench_std_sort(b: &mut Bencher, len: usize) {
        let v = generate_unsorted(len);
        b.iter(|| {
            let mut my_v = v.clone();
            my_v.sort();
        });
    }

    #[bench]
    fn bench_std_sort_small(b: &mut Bencher) {
        bench_std_sort(b, 1024);
    }

    #[bench]
    fn bench_radix_sort_small(b: &mut Bencher) {
        bench_radix_sort(b, 1024);
    }

    #[bench]
    fn bench_faster_radix_sort_small(b: &mut Bencher) {
        bench_faster_radix_sort(b, 1024);
    }

    #[bench]
    fn bench_std_sort_big(b: &mut Bencher) {
        bench_std_sort(b, 512 * 1024);
    }

    #[bench]
    fn bench_radix_sort_big(b: &mut Bencher) {
        bench_radix_sort(b, 512 * 1024);
    }

    #[bench]
    fn bench_faster_radix_sort_big(b: &mut Bencher) {
        bench_faster_radix_sort(b, 512 * 1024);
    }
}
