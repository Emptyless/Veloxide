use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct UserServices {}

impl UserServices {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UserServices {
    fn default() -> Self {
        Self::new()
    }
}
