use shared::action::Action;

pub trait ToAction {
    fn to_action(self) -> Action;
}

impl ToAction for &str {
    fn to_action(self) -> Action {
        match self {
            "identify" => Action::Identify,
            "roll" => Action::Roll,
            "buy_all" => Action::BuyAll,
            _ => Action::Invalid,
        }
    }
}
