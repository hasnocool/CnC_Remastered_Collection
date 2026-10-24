//! Engineer building capture (RA special).
//!
//! Per BDATA: `IsCaptureable` gate controls which structures can be captured.
use cnc_core::ClassId;

/// Whether a structure class is captureable according to BDATA.
pub fn is_captureable(class_id: ClassId) -> bool {
    matches!(
        class_id,
        ClassId::PowerPlant
            | ClassId::Barracks
            | ClassId::Refinery
            | ClassId::WarFactory
            | ClassId::Helipad
            | ClassId::Airfield
            | ClassId::NavalYard
            | ClassId::SubPen
            | ClassId::Silo
            | ClassId::ConstructionYard
    )
}

/// Engineer captures a captureable enemy building and is consumed.
pub fn capture_building(class_id: ClassId) -> bool {
    is_captureable(class_id)
}
