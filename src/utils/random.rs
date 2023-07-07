use rand::Rng;

pub enum RandKind {
    Range,
    Full,
}

pub fn gen_vec_rand_int(n: usize, kind: RandKind) -> Vec<isize> {
    // Generate random integers within a range or not
    let mut rng = rand::thread_rng();
    let mut vec: Vec<isize> = Vec::new();
    if let RandKind::Range = kind {
        for _ in 0..n {
            vec.push(rng.gen_range(-10000000isize..10000000isize));
        }
    } else {
        for _ in 0..n {
            vec.push(rng.gen::<isize>());
        }
    }
    vec
}
