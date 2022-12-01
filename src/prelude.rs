pub use crate::grid::*;
pub use itertools::Itertools;
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        ::std::println!($($arg)*);
    }};
}
