use strum_macros::{Display, EnumIter};

/// Serves as `BurpSuite` attack type, but better lol
#[derive(Default, Clone, PartialEq, Display, EnumIter)]
pub enum AttackType {
    #[default]
    /// Sniper attack ahh
    Sequential,
    /// runs synchronous to other lists
    Parallel,
    /// cartesian product of lists (orgy)
    Matrix,
}
