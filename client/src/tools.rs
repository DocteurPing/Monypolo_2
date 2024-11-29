use shared::action::Action;

pub trait ToAction {
    fn to_action(self) -> Action;
}

impl ToAction for &str {
    fn to_action(self) -> Action {
        match self {
            "identify" => Action::Identify,
            "quit" => Action::Quit,
            _ => Action::Invalid,
        }
    }
}