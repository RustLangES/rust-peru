use reactive_stores::Store;


#[derive(Clone, Default, Debug, Store)]
pub struct AppState {
    pub is_open: bool,
}
