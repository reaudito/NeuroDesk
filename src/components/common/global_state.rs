use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    pub account_state: String,
}