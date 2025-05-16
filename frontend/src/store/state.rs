
#[derive(Clone, PartialEq, Default)]
pub struct GlobalState {
    pub token: Option<String>,
    pub theme: String,
}

#[derive(Clone)]
pub enum GlobalAction {
    SetToken(String),
    SetTheme(String),
}