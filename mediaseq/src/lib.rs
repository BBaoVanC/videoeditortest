pub trait Container {
}
pub struct MkvContainer {
}

pub trait MediaPoint {

}

pub struct MediaExtend {
    inner: Box<dyn MediaPoint>,
    length_before:
    length_after:
}
