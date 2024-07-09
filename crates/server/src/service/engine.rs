use crate::{config::Config, dao::entity::Script, errors::AppResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write};
use tera::{Context, Result as TeraResult, Tera, Value};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveData {
    pub name: String,
    pub environment: String,
    pub description: String,
    pub pre_processors: Vec<StepInfo>,
    pub steps: Vec<StepInfo>,
    pub after_processors: Vec<StepInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub framework: Framework,
    pub attach_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Framework {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepInfo {
    pub position: i32,
    pub action: String,
    pub selector: Option<String>,
    pub attach_info: Option<HashMap<String, String>>,
}

fn remove_empty_lines(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("");
    let cleaned = s
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(Value::String(cleaned))
}

pub async fn generator(script: DriveData) -> AppResult<Script> {
    let config = Config::parse("./config.toml").expect("Failed to parse configuration file");

    // initialize Tera template engine.
    let mut tera = Tera::new(format!("{}/*.js", config.storage.template_path).as_str())?;

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
    let rendered = tera.render("cypress_template.cy.js", &ctx)?;

    // generate filename dynamically.
    let filepath = format!("{}/cypress_test.cy.js", config.storage.script_path);
    let mut file = File::create(&filepath)?;
    file.write_all(rendered.as_bytes())?;
    Ok(Script {
        case_id: 0,
        path: filepath,
        environment: "".into(),
        created_at: Utc::now(),
        created_by: Uuid::nil(),
    })
}

#[allow(dead_code)]
pub async fn doctor(_env: Environment) -> AppResult<()> {
    /* ensure environment available */
    // match env.framework {}
    Ok(())
}
