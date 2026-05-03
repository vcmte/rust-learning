pub mod hash;
pub mod string;
pub mod vector;

use hash::map_demonstration as hmap_dem;
use string::demonstration as str_dem;
use vector::demonstration as vec_dem;

fn main() {
    hmap_dem();
    str_dem();
    vec_dem();
}
