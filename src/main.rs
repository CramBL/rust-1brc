pub mod util;

#[cfg(feature = "parallel")]
pub mod parallel;
#[cfg(feature = "sequential")]
pub mod sequential;

fn main() {
    #[cfg(feature = "sequential")]
    sequential::do_work();
    #[cfg(feature = "parallel")]
    parallel::do_work();
}
