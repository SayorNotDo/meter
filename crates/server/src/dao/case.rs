#[derive(Debug)]
pub struct CaseDao<'a> {
    client: &'a db::Client,
}

impl<'a> CaseDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        CaseDao { client }
    }
}