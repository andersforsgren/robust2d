
#[link(name = "predicates")]
extern "C" {
    pub fn exactinit();
    pub fn incircle(a: *const f64, b: *const f64, c: *const f64, d: *const f64) -> f64;
    pub fn orient2d(a: *const f64, b: *const f64, c: *const f64) -> f64;
}
