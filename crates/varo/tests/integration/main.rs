use varo::{Rng, Score, Varo, optimize};

#[test]
fn test_optimize() {
    struct Foo {
        x: f32,
    }

    impl Varo for Foo {
        fn next(digest: &mut varo::Rng) -> Self {
            let x = varo::rng_gen_f32(digest) * 10.0;
            Foo { x }
        }
    }

    fn evaluate(foo: Foo) -> Score {
        let x = foo.x;
        let score = -0.9 * x.powi(3) + 2.6 * x.powi(2) - 4.0 * x;
        score.into()
    }

    let mut rng = varo::rng_new();
    let optimization_result = optimize(evaluate, &mut rng, 10);
    assert_eq!(optimization_result.values.len(), 10);
    let scores: Vec<f32> = optimization_result.values.iter().map(|pair| pair.1).collect();
    for i in 0..scores.len() - 1 {
        assert!(scores[i] > scores[i + 1]);
    }
    println!();
}
