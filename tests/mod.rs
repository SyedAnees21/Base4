use rand::{
    Rng,
    distr::{Uniform, uniform::SampleUniform},
};

use base4::{Base4, Base4Int};

fn random_ints<T>(len: usize) -> Vec<T>
where
    T: SampleUniform + Copy + From<u8>,
{
    use rand;
    let rng = rand::rng();
    let sampler = Uniform::<T>::new(T::from(0_u8), T::from(4_u8)).unwrap();
    rng.sample_iter(sampler).take(len).collect()
}

#[test]
fn base4_int_smoke() {
    let mut base4_integer = Base4Int::new();

    base4_integer.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);

    println!("{:?}", base4_integer);

    assert!(0 == base4_integer.pop() as u64);
    assert!(1 == base4_integer.pop() as u64);
    assert!(2 == base4_integer.pop() as u64);
    assert!(3 == base4_integer.pop() as u64);
    assert!(2 == base4_integer.pop() as u64);
    assert!(1 == base4_integer.pop() as u64);
    assert!(0 == base4_integer.pop() as u64);

    base4_integer.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);

    assert!(vec![0_u64, 1, 2, 3, 2, 1, 0] == base4_integer.pop_all());
}

#[test]
fn peek_from_base4_int() {
    let mut ints = random_ints::<u64>(70);
    let mut base4_integer = Base4Int::new();

    base4_integer.push_all(ints.as_slice());

    (0..70).for_each(|i| assert!(ints[i] == base4_integer.peek_at(i)));

    ints.clear();
    ints = random_ints(128);

    let mut base4_integer = Base4Int::new();
    base4_integer.push_all(ints.as_slice());

    (0..128).for_each(|i| assert!(ints[i] == base4_integer.peek_at(i)));

    ints.clear();
    ints = random_ints(256);

    let mut base4_integer = Base4Int::new();
    base4_integer.push_all(ints.as_slice());

    (0..256).for_each(|i| assert!(ints[i] == base4_integer.peek_at(i)));
}

#[test]
fn push_pop_base4_int() {
    let ints = random_ints::<u64>(128);
    let mut base4_integer = Base4Int::new();

    base4_integer.push_all(ints.as_slice());

    assert!(base4_integer.total_blocks() == 2);
    assert!(base4_integer.total_len() == 128);

    base4_integer.push(2_u64);

    assert!(base4_integer.total_blocks() == 3);
    assert!(base4_integer.total_len() == 129);
    assert!(base4_integer.peek_at::<u64>(128) == 2);
    assert!(base4_integer.pop() == 2);
    assert!(base4_integer.total_blocks() == 2);
    assert!(base4_integer.total_len() == 128);
}

#[test]
fn base4_codec() {
    let mut base4_integer = Base4::new();

    base4_integer.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);

    println!("{:?}", base4_integer);

    assert!(0 == base4_integer.pop() as u64);
    assert!(1 == base4_integer.pop() as u64);
    assert!(2 == base4_integer.pop() as u64);
    assert!(3 == base4_integer.pop() as u64);
    assert!(2 == base4_integer.pop() as u64);
    assert!(1 == base4_integer.pop() as u64);
    assert!(0 == base4_integer.pop() as u64);

    base4_integer.push_all(&[0_u64, 1, 2, 3, 2, 1, 0]);

    assert!(vec![0_u64, 1, 2, 3, 2, 1, 0] == base4_integer.pop_all());
}

#[test]
fn peek_from_base4() {
    let mut ints = random_ints::<u64>(10);
    let mut base4_integer = Base4::new();

    base4_integer.push_all(ints.as_slice());

    (0..10).for_each(|i| assert!(ints[i] == base4_integer.peek_at(i)));

    ints.clear();
    ints = random_ints(64);
    let mut base4_integer = Base4::new();

    base4_integer.push_all(ints.as_slice());

    (0..64).for_each(|i| assert!(ints[i] == base4_integer.peek_at(i)));
}

#[test]
fn base4_codec_limits() {
    let mut ints = random_ints(12);
    let mut base4_integer = Base4::new();

    base4_integer.push_all(ints.as_slice());

    assert!(ints == base4_integer.pop_all());

    ints.clear();
    ints = random_ints(64);
    base4_integer.push_all(ints.as_slice());

    assert!(base4_integer.pop_all::<u64>() == ints);

    ints.clear();
    ints = random_ints(65);

    base4_integer.push_all(ints.as_slice());

    assert!(ints != base4_integer.pop_all());
}

#[test]
#[should_panic = "Attempt to pop an empty Base4-Integer"]
fn base4_int_empty() {
    use rand;
    let mut rng = rand::rng();
    let mut ints = vec![];
    let mut base4_integer = Base4Int::new();

    (0..70).for_each(|_| ints.push(rng.random_range(0..4_u64)));

    base4_integer.pop();
    base4_integer.push_all(ints.as_slice());

    base4_integer.peek_at::<u8>(70);
}

#[test]
#[should_panic = "peek_at: index 70 out of bounds (size=70)"]
fn base4_int_oob() {
    use rand;
    let mut rng = rand::rng();
    let mut ints = vec![];
    let mut base4_integer = Base4Int::new();

    (0..70).for_each(|_| ints.push(rng.random_range(0..4_u64)));

    base4_integer.push_all(ints.as_slice());

    base4_integer.peek_at::<u8>(70);
}

#[test]
#[should_panic = "Base4Int only accepts value bounded within 0..=3"]
fn base4_int_unbounded() {
    let mut base4_integer = Base4Int::new();
    base4_integer.push(4_u64);
}

#[test]
fn bit_shift_multiplication() {
    let a = 4 * 1;
    let b = 1 << 2;
    assert!(a == b);

    let a = 4 * 2;
    let b = 2 << 2;
    assert!(a == b);

    let a = 4 * 3;
    let b = 3 << 2;
    assert!(a == b);

    let a = 4 * 4;
    let b = 4 << 2;
    assert!(a == b);

    let a = 1 / 4;
    let b = 1 >> 2;
    assert!(a == b);

    let a = 8 / 4;
    let b = 8 >> 2;
    assert!(a == b);

    let a = 12 / 4;
    let b = 12 >> 2;
    assert!(a == b);

    let a = 16 / 4;
    let b = 16 >> 2;
    assert!(a == b);

    let encoded = base4_encode(&[0, 1, 2, 3, 2, 1, 0]);
    println!("{encoded}");
    let decoded = base4_decode(encoded, 7);
    println!("{:?}", decoded);
}

fn base4_encode(ints: &[usize]) -> u128 {
    let mut n = 0;
    for int in ints {
        n = n * 4 + (*int as u128);
    }
    n
}

fn base4_decode(n: u128, size: usize) -> Vec<u64> {
    let mut ints = Vec::with_capacity(size);
    let mut N = n;
    for _ in 0..size {
        ints.push((N % 4) as u64);
        N /= 4;
    }
    ints
}
