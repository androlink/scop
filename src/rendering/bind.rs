pub fn bind(value: &impl Bindable) {
    value.bind();
}

pub fn unbind(value: &impl Bindable) {
    value.unbind();
}

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self) {}
}
