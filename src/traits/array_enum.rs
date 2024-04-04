pub trait ArrayEnum {
    fn get_next(&self) -> Self;
    fn get_prev(&self) -> Self;
    fn get_array_form() -> Vec<Self>
    where
        Self: std::marker::Sized;
}
