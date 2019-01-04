mod hitable;
mod scattering;

pub use self::hitable::*;
pub use self::scattering::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
