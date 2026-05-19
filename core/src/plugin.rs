// core/src/plugin.rs
pub trait Plugin {
    const NAME: &'static str;
    fn build(&self, app: &mut crate::app::App);
}
