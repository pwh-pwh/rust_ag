pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(value: u64) -> Self {
        Duration(
            (value as f64)/31557600f64
        )
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d:&Duration) -> f64 {
        d.0/Self::period()
    }
}

macro_rules! planet {
    ($n:ident,$e:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn period() -> f64 {
                $e
            }
        }
    }
}
planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_er() {
        let duration = Duration::from(1_000_000_000);
        let during = Earth::years_during(&duration);
        println!("{}",during);
    }

    #[test]
    fn test_mar() {
        let duration = Duration::from(2_134_835_688);
        let during = Mercury::years_during(&duration);
        println!("{}",during);
    }

    #[test]
    fn test_ve() {
        let duration = Duration::from(189_839_836);
        let during = Venus::years_during(&duration);
        println!("{}",during);
    }

}