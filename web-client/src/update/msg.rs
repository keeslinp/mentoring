use model::Route;

#[derive(Debug)]
pub enum Msg {
    /// Changes the route to the given one.
    ChangeRoute(Route),

    /// Represents a change in the hash.
    HashChange(String),

    /// Notifies that an update to the site is pending.
    UpdateNotify,
}
