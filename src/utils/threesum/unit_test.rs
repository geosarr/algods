#[cfg(test)]
mod tests {
    use super::super::ThreeSum;
    use crate::utils::{gen_vec_rand_int, RandKind};
    use std::thread;
    use std::thread::JoinHandle;

    #[test]
    fn test_threesum() {
        let vec = gen_vec_rand_int(20, RandKind::Range);
        let n = vec.len();
        let mut threesum = ThreeSum {
            target: 0,
            vec: vec,
        };
        let mut handles: Vec<JoinHandle<isize>> = Vec::new();
        for i in 0..n {
            let a = threesum.vec[i];
            for j in (i + 1)..n {
                let b = threesum.vec[j];
                for k in (j + 1)..n {
                    let c = threesum.vec[k];
                    let handle = thread::spawn(move || a + b + c);
                    handles.push(handle);
                }
            }
        }
        let sums = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<isize>>();
        for s in &sums {
            threesum.target = *s;
            assert!(1 <= threesum.run());
        }
    }
}
