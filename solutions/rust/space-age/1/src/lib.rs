// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.0 as f64 / (24 * 60 * 60) as f64) / 365.25;
        earth_years / Self::earth_years()
    }

    fn earth_years() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! earth_years {
    ($planet:ty, $years:expr) => {
        impl Planet for $planet {
            fn earth_years() -> f64 {
                $years
            }
        }
    };
}

earth_years!(Mercury, 0.2408467);
earth_years!(Venus, 0.61519726);
earth_years!(Earth, 1.0);
earth_years!(Mars, 1.8808158);
earth_years!(Jupiter, 11.862615);
earth_years!(Saturn, 29.447498);
earth_years!(Uranus, 84.016846);
earth_years!(Neptune, 164.79132);
