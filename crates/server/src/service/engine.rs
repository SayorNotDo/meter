use std::{
    fs::File,
    io::{Result, Write},
};
use tera::{Context, Tera};

#[derive(serde::Serialize)]
pub struct CaseInfo {
    name: String,
    env: String,
    description: String,
    pre_processors: Vec<()>,
    steps: Vec<()>,
    after_processors: Vec<()>,
}

#[allow(dead_code)]
pub async fn generator(case: CaseInfo) -> Result<()> {
    // initialize Tera template engine.
    let tera = match Tera::new("/static/templates/*.js") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {e:?}");
            std::process::exit(1);
        }
    };

    // create template context.
    let mut ctx = Context::new();

    // different scenes & parameters
    ctx.insert("name", &case.name);
    ctx.insert("description", &case.description);
    ctx.insert("pre_processors", &case.steps);

    // render template
    let rendered = match tera.render("cypress_template.cy.js", &ctx) {
        Ok(t) => t,
        Err(e) => {
            println!("Rendering error(s): {}", e);
            std::process::exit(1);
        }
    };

    // generate filename dynamically.
    let filename = format!("");
    let mut file = File::create(&filename)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}
