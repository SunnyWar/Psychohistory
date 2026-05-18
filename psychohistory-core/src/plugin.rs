pub trait Plugin {
    fn build(&self, app: &mut crate::app::App);
}
