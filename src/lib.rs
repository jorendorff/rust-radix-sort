#![feature(test)]

#[cfg(test)] extern crate test;

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

    #[test]
    fn it_works() {
        let mut data = vec![512, 4, 3, -1, 256, 1, 6];
        radix_sort(&mut data);
        assert_eq!(data, vec![-1, 1, 3, 4, 6, 256, 512]);
    }

    // This function returns the same vector of random-looking values each time
    // it's called.
    fn generate_unsorted() -> Vec<i32> {
        let mut v = vec![0; 512 * 1024];

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

    #[bench]
    fn radix_sort_big_data(b: &mut Bencher) {
        let v = generate_unsorted();
        b.iter(|| {
            let mut my_v = v.clone();
            radix_sort(&mut my_v);
        });
    }

    #[bench]
    fn std_sort_big_data(b: &mut Bencher) {
        let v = generate_unsorted();
        b.iter(|| {
            let mut my_v = v.clone();
            my_v.sort();
        });
    }
}
