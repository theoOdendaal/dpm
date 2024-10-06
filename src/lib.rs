//  --- Modules ---
pub mod conventions {
    pub mod business_day;
    pub mod day_count;
}

pub mod core {
    pub mod sequence;
    pub mod solver;
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
}

pub mod resources {
    pub mod holidays;
}

pub mod time {
    pub mod periods;
}
