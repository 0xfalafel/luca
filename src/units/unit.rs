use compact_str::CompactString;

// pub type ConversionFactor = f64;
pub type CanonicalName = CompactString;

/// A unit can either be a base/fundamental unit or it is derived from another unit.
/// In the latter case, a conversion factor to the defining unit has to be specified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitKind {
    Base,
    // Derived(ConversionFactor, Unit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub symbol: CompactString,
    pub name: CanonicalName,
    kind: UnitKind,
}


impl Unit {
    pub fn percent() -> Unit {
        Unit { symbol: "%".into(), name: "percent".into(), kind: UnitKind::Base }
    }

    #[allow(unused)]
    pub fn meter() -> Unit {
        Unit { symbol: "m".into(), name: "meter".into(), kind: UnitKind::Base }
    }

    pub fn euro() -> Unit {
        Unit { symbol: "€".into(), name: "euro".into(), kind: UnitKind::Base }
    }

    pub fn dollar() -> Unit {
        Unit { symbol: "$".into(), name: "dollar".into(), kind: UnitKind::Base }
    }
}