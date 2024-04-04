pub trait ArrayEnum {
    fn get_next(&self) -> Self;
    fn get_prev(&self) -> Self;
}
