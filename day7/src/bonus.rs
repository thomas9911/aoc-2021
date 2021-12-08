
// nice file with extra functions, I thought I needed but didnt


fn secant_method_inner<F: Fn(f64) -> f64>(f: &F, x0: f64, x1: f64) -> f64 {
    let fx0 = f(x0);
    let fx1 = f(x1);
    x1 - ((x0 - x1) / (fx0 - fx1)) * fx1
}

fn secant_method<F: Fn(f64) -> f64 >(f: F, mut x0: f64, mut x1: f64, rounds: usize) -> f64  {
    for _ in 0..rounds {

        let x2 = {secant_method_inner(&f, x0, x1)};
        if x2.is_nan() {
            return x1
        }
        x0 = x1;
        x1 = x2;
    }

    x1
}

#[test]
fn test_secant_sqrt_3() {
    let f = |x| {(x*x) - 3.0};

    let x = secant_method(f, 0.0, 2.0, 10);

    assert!((x - 3.0f64.sqrt()).abs() < 0.0000001 )
}

#[test]
fn test_secant_sqrt_2() {
    let f = |x| {(x*x) - 2.0};

    let x = secant_method(f, 0.0, 2.0, 10);

    assert!((x - 2.0f64.sqrt()).abs() < 0.0000001 )
}

fn harmonic_mean(input: &[f64]) -> f64{
    let size = input.len();
    let s: f64 = input.iter().map(|x| (x+1.0).recip()).sum();
    (size as f64 / s).trunc()
}

fn harmonic_mean_of_counter(input: &BTreeMap<i64, i64>) -> f64 {
    input.iter().fold(0.0f64, |mut acc, (value, amount)| {
        acc += ((value+1)  as f64).recip() * *amount as f64;
        acc
    })
}

#[test]
fn test_harmonic_mean() {
    let input = [16,1,2,0,4,2,7,1,2,14];

    let x = harmonic_mean(&input.map(|x| x as f64));

    assert_eq!(2.0, x);
}
