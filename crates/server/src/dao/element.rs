#[derive(Debug)]
pub struct ElementDao<'a> {
    pub client: &'a db::Client,
}

impl<'a> ElementDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        client
    }
}
