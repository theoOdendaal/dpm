// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

// 3.
// All functions should take Vec<f64> as self.

fn main() {
    /*
    let n = vec![0.5, 0.7];
    let pv = vec![0.97, 0.96];
    let conventions = InterestConventions::Simple;
    let res: f64 = conventions.rate(&n, &pv);
    println!("{:?}", res);
    */

    /*
    let df = vec![0.996489, 0.991306, 0.984494, 0.975616]; //, 0.964519];
    let n = vec![0.5, 0.5, 0.5, 0.5]; //, 0.5];
    let convention = InterestConventions::Discrete(
        dpm::interest::ops::DiscreteCompoundingFrequencies::SemiAnnually,
    );
    let swap_rate = 0.07;
    let swap_point = (0.5, swap_rate);
    let df_points = (n.as_slice(), df.as_slice());

    let res = swap_to_discount(&convention, &swap_point, &df_points);

    let all_ns = vec![0.5, 0.5, 0.5, 0.5, 0.5];
    let mut all_dfs = vec![0.996489, 0.991306, 0.984494, 0.975616];
    all_dfs.push(res);

    let testing = discount_and_swap_check(&convention, &swap_rate, &(&all_ns, &all_dfs));

    println!("{:?}", testing);
    println!("{:?}", &res);
    */

    // Forward rate solver.
    /*
    let first = (90.0 / 365.0, 0.982262730598449);
    let second = (181.0 / 365.0, 0.962226033210754);
    let convention = InterestConventions::Simple;
    let f = discount_to_forward(&convention, &first, &second);
    println!("{:?}", f);
    */

    // Swap rate solver.
    /*
    let df = [0.996489, 0.991306, 0.984494, 0.975616, 0.964519];
    let n = [0.5, 0.5, 0.5, 0.5, 0.5];
    let df = &df[..2];
    let n = &n[..df.len()];
    let points = (n, df);
    let convention = InterestConventions::Discrete(
        dpm::interest::ops::DiscreteCompoundingFrequencies::SemiAnnually,
    );
    let close = |a: f64| discount_and_swap_check(&convention, &a, &points);
    let res_1 = f64::solve(&0.0, close);
    dbg!(res_1);
    dbg!(discount_and_swap_check(&convention, &res_1, &points));
    */

    // Solver.
    /*
    let n = 0.75;
    let comp = dpm::interest::ops::DiscreteCompoundingFrequencies::Annually;
    let m: f64 = comp.into();

    let f_closure: Box<dyn Fn(&f64) -> f64 + '_> =
        Box::new(move |a: &f64| (1.0 + a / m).powf(n * m));

    let b_closure: Box<dyn Fn(&f64) -> f64 + '_> =
        Box::new(move |a: &f64| (n * m) * (1.0 + a / m).powf((n * m) - 1.0));

    let target = 1.12;

    let res = solver::Discrete::solve(&f_closure, &b_closure, &target);
    println!("{:?}", res);

    let res_2 = InterestConventions::Discrete(comp);
    let res_2 = res_2.fv(&n, &res);
    println!("{:?}", res_2);
    */
}
