use compact_str::CompactString;

pub type ConversionFactor = f64;
pub type CanonicalName = CompactString;

/// A unit can either be a base/fundamental unit or it is derived from another unit.
/// In the latter case, a conversion factor to the defining unit has to be specified.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitKind {
    Base,
    Derived(ConversionFactor, Box<Unit>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub symbol: CompactString,
    pub name: CanonicalName,
    kind: UnitKind,
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

    pub fn conversion_factor(&self, dest_unit: &Unit) -> Option<f64> {
        let (base_unit_self, convertion_factor_self) = self.base_and_factor();
        let (base_unit_other, convertion_factor_other) = dest_unit.base_and_factor();

        // We can only do conversion if units are derived from each other (aka: cm to m)
        if base_unit_self != base_unit_other {
            return None
        }

        Some(convertion_factor_self / convertion_factor_other)
    }
}

impl Unit {
    pub fn percent() -> Unit {
        Unit { symbol: "%".into(), name: "percent".into(), kind: UnitKind::Base }
    }

    pub fn meter() -> Unit {
        Unit { symbol: "m".into(), name: "meter".into(), kind: UnitKind::Base }
    }

    pub fn centimeter() -> Unit {
        Unit {
            symbol: "cm".into(), name: "centimeter".into(),
            kind: UnitKind::Derived(0.01, Box::new(Unit::meter()))
        }
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
        assert_eq!(Some(100.0), m.conversion_factor(cm));
    }

    #[test]
    fn cm_to_m() {
        let m = Unit::meter();
        let cm = Unit::centimeter();
        assert_eq!(Some(0.01), cm.conversion_factor(m));
    }
}