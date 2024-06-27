use crate::{config::Config, errors::AppResult};
use std::{
    collections::HashMap,
    fs::File,
    io::{Result, Write},
};
use tera::{Context, Result as TeraResult, Tera, Value};
use tracing::info;

use crate::dao::entity::Script;

fn remove_empty_lines(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("");
    let cleaned = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(Value::String(cleaned))
}

pub async fn generator(script: Script) -> AppResult<String> {
    let config = Config::parse("./config.toml").expect("Failed to parse configuration file");

    // initialize Tera template engine.
    let mut tera = match Tera::new(format!("{}/*.js", config.storage.template_path).as_str()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {e:?}");
            std::process::exit(1);
        }
    };

    // register customized filter.
    tera.register_filter("remove_empty_lines", remove_empty_lines);

    // create template context.
    let mut ctx = Context::new();

    // different scenes & parameters
    ctx.insert("name", &script.name);
    ctx.insert("description", &script.description);
    ctx.insert("pre_processors", &script.pre_processors);
    ctx.insert("after_processors", &script.after_processors);
    ctx.insert("case_steps", &script.steps);

    // render by template engine.
    let rendered = match tera.render("cypress_template.cy.js", &ctx) {
        Ok(t) => t,
        Err(e) => {
            println!("Rendering error(s): {}", e);
            std::process::exit(1);
        }
    };

    // generate filename dynamically.
    let filename = format!("{}/cypress_test.cy.js", config.storage.script_path);
    let mut file = File::create(&filename)?;
    file.write_all(rendered.as_bytes())?;
    Ok(filename)
}
