// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

// 3.
// All functions should take Vec<f64> as self.

// FIXME How to handle multiple compounding frequencies within a single payment interval?

use dpm::resources::holidays::CountryCache;

fn main() {
    let country_cache = CountryCache::new();
    println!("{:?}", country_cache);

    //let res = CountryCache::load_config().unwrap();
    //println!("{:?}", res.is_saved("ZA", "2023"));
}
