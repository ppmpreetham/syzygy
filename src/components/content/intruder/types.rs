use strum_macros::{Display, EnumIter};

/// Serves as `BurpSuite` attack type, but better lol
#[derive(Clone, PartialEq, Display, EnumIter)]
pub enum AttackType{
  /// Sniper attack ahh
  Sequential,
  /// runs synchronous to other lists
  Parallel,
  /// cartesian product of lists (orgy)
  Matrix,
}
