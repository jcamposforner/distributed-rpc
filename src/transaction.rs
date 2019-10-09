pub struct Transaction<'t> {
    pub id: usize,
    pub from: &'t std::net::SocketAddr,
    pub to: &'t str,
    pub event_type: &'t str
}