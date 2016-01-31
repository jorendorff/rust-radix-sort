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

#[test]
fn it_works() {
    let mut data = vec![512, 4, 3, -1, 256, 1, 6];
    radix_sort(&mut data);
    assert_eq!(data, vec![-1, 1, 3, 4, 6, 256, 512]);
}
