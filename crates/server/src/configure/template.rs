use tera::Tera;

use crate::dto::EmailTemplate;

#[derive(Clone)]
pub struct EmailTemplateEngine {
    tera: Tera,
}

impl EmailTemplateEngine {
    pub fn new(path: &str) -> tera::Result<Self> {
        Ok(Self {
            tera: Tera::new(path)?,
        })
    }

    pub fn render(&self, tempalte: &EmailTemplate) -> Result<String, tera::Error> {
        let (ctx, path) = tempalte.get();
        self.tera.render(path, &ctx)
    }
}
