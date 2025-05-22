use compact_str::CompactString;
use std::hash::{Hash, Hasher};

pub type ConversionFactor = f64;
pub type CanonicalName = CompactString;

/// A unit can either be a base/fundamental unit or it is derived from another unit.
/// In the latter case, a conversion factor to the defining unit has to be specified.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitKind {
    Base,
    Derived(ConversionFactor, Box<Unit>),
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub symbol: CompactString,
    pub name: CanonicalName,
    kind: UnitKind,
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Unit {}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Unit {
    #[allow(unused)]
    fn is_base_unit(&self) -> bool {
        self.kind == UnitKind::Base
    }

    /// Return the name of the base, and conversion factor
    fn base_and_factor(&self) -> (&CanonicalName, f64) {
        match &self.kind {
            UnitKind::Derived(conversion_factor, base_unit) => (&base_unit.name, *conversion_factor),
            UnitKind::Base => (&self.name, 1.0)
        }
    }

    pub fn can_be_converted_to(&self, dest_unit: &Unit) -> bool {
        let (base_unit_self, _ ) = self.base_and_factor();
        let (base_unit_other, _) = dest_unit.base_and_factor();

        base_unit_self == base_unit_other
    }


    pub fn conversion_factor(&self, dest_unit: &Unit) -> Option<f64> {
        let (base_unit_self, convertion_factor_self) = self.base_and_factor();
        let (base_unit_other, convertion_factor_other) = dest_unit.base_and_factor();

        // We can only do conversion if units are derived from each other (aka: cm to m)
        if base_unit_self != base_unit_other {
            return None
        }

        Some(convertion_factor_self / convertion_factor_other)
    }

    /// Create derived units like kilometer from meter.
    pub fn with_prefix(&self, prefix: &str) -> Unit {
        match prefix {
            "kilo" => Unit {
                symbol: format!("k{}", self.symbol).into(), name: format!("kilo{}", self.name).into(),
                kind: UnitKind::Derived(1000.0, Box::new(self.clone()))
            },
            "deci" => Unit {
                symbol: format!("d{}", self.symbol).into(), name: format!("deci{}", self.name).into(),
                kind: UnitKind::Derived(0.1, Box::new(self.clone()))
            },
            "centi" => Unit {
                symbol: format!("c{}", self.symbol).into(), name: format!("centi{}", self.name).into(),
                kind: UnitKind::Derived(0.01, Box::new(self.clone()))
            },
            "milli" => Unit {
                symbol: format!("m{}", self.symbol).into(), name: format!("milli{}", self.name).into(),
                kind: UnitKind::Derived(0.001, Box::new(self.clone()))
            },
            _ => panic!("Unknow prefix")
        }
    }
}

impl Unit {
    pub fn percent() -> Unit {
        Unit { symbol: "%".into(), name: "percent".into(), kind: UnitKind::Base }
    }

    pub fn meter() -> Unit {
        Unit { symbol: "m".into(), name: "meter".into(), kind: UnitKind::Base }
    }
    pub fn kilometer()  -> Unit { Unit::meter().with_prefix("kilo") }
    pub fn decimeter()  -> Unit { Unit::meter().with_prefix("deci") }
    pub fn centimeter() -> Unit { Unit::meter().with_prefix("centi") }
    pub fn millimeter() -> Unit { Unit::meter().with_prefix("milli") }

    pub fn second() -> Unit {
        Unit { symbol: "s".into(), name: "second".into(), kind: UnitKind::Base }
    }
    pub fn millisecond() -> Unit { Unit::second().with_prefix("milli") }
    pub fn minute() -> Unit {
        Unit {
            symbol: "min".into(), name: "minute".into(),
            kind: UnitKind::Derived(60.0, Box::new(Self::second())) }
    }
    pub fn hour() -> Unit {
        Unit {
            symbol: "h".into(), name: "hour".into(),
            kind: UnitKind::Derived(3600.0, Box::new(Self::second())) }
    }

    pub fn euro() -> Unit {
        Unit { symbol: "€".into(), name: "euro".into(), kind: UnitKind::Base }
    }

    pub fn dollar() -> Unit {
        Unit { symbol: "$".into(), name: "dollar".into(), kind: UnitKind::Base }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m_to_cm() {
        let m = Unit::meter();
        let cm = Unit::centimeter();
        assert_eq!(Some(100.0), m.conversion_factor(&cm));
    }

    #[test]
    fn cm_to_m() {
        let m = Unit::meter();
        let cm = Unit::centimeter();
        assert_eq!(Some(0.01), cm.conversion_factor(&m));
    }
}