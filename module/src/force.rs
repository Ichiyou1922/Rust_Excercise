// 外部から使うためpubをつける
pub enum Force {
    Gravity,
    Impulse(f64),
    Drag(f64),
}
