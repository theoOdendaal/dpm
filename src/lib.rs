// ---  !!! Important
// TODO add detailed documentation for each module.
// TODO ensure consistency of documentation accross all modules !!!
// TODO i really like the design pattern of the convention::business_day and convention::day_count. Reuse this where required.
// TODO when implementing a trait for various generics, no two implementation can produce the same output, as this creates ambiguity.
// Make sure Default trait in implemented for all enums, unless explicit reason exist why this is not appropriate.

//  --- Miscellaneous
// TODO all new functions should be 'const'.
// TODO use &[T] rather than &Vec<T>.
// TODO implement CLI functionality.
// TODO add unit tests for each module.
// TODO unit tests should also test errors.

//  --- Modules ---
pub mod conventions {
    pub mod business_day;
    pub mod day_count;
}

pub mod core {
    pub mod sequence;
}

pub mod interest {
    pub mod ops;
    pub mod term_structure;
    pub mod types;
}

pub mod iso {
    pub mod iso3166;
}

pub mod macros {
    pub mod approx;
    pub mod tprint;
}

pub mod math {
    pub mod interpolation;
    pub mod poly;
    pub mod solver;
    pub mod splines;
}

pub mod resources {
    pub mod holidays;
}

pub mod time {
    pub mod periods;
}
