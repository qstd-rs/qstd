use qstd_core::aliases::Index;

pub const RELEASED: Index = Index::MAX;

pub const fn is_released(index: Index) -> bool {
    index == RELEASED
}

pub const fn is_active(index: Index) -> bool {
    !is_released(index)
}
