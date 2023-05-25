pub enum UpdateStatus {
    All,
    Some,
    None,
}

impl UpdateStatus {
    pub fn bool(&self) -> bool {
        match self {
            Self::All | Self::Some => true,
            Self::None => false,
        }
    }
}

impl From<UpdateStatus> for bool {
    fn from(val: UpdateStatus) -> Self {
        val.bool()
    }
}
pub trait Module {
    fn get_string(&self) -> &str;
    fn update(&mut self) -> UpdateStatus;
    fn update_interval(&self) -> u64;
}
