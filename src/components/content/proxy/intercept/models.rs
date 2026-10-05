use strum::FromRepr;
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub(super) enum ForwardAction {
    Forward = 0,
    ForwardAll = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub(super) enum DropAction {
    Drop = 0,
    DropAll = 1,
}
