#[macro_export]
macro_rules! normalize {
    ($r: expr) => {
        ($r as f32) / (255f32)
    };
    ($r: expr, $factor:  expr) => {
        ($r as f32) / ($factor as f32)
    };
}
