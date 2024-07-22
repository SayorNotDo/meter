use crate::dao::entity::Machine;
use crate::{config::Config, dao::entity::Script, errors::AppResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::path::Path;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};
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
pub enum Framework {
    Cypress,
    Unknown,
}

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
pub async fn doctor(env: Environment) -> AppResult<()> {
    /* ensure environment available */
    match env.framework {
        Framework::Cypress => {
            println!("cypress running environment.");
            let machine: Machine = Machine {
                addr: "192.168.50.234:22".into(),
                user: "root".into(),
                password: "123456".into(),
            };
            let script: &Path = Path::new("");
            let _ = doctor_script(machine, script).await?;
        }
        Framework::Unknown => println!("unsupported framework"),
    }

    Ok(())
}

pub async fn doctor_script(machine: Machine, script: &Path) -> AppResult<String> {
    let file_name = script.file_name().expect("Failed to get script name");
    let remote_script_path = Path::new(file_name);
    /* build tcp connection */
    let tcp = TcpStream::connect(&machine.addr)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    /* authentication */
    session.userauth_password(&machine.user, &machine.password)?;

    /* creat SFTP channel */
    let sftp = session.sftp()?;
    /* get script and upload to specific machine */
    let script_file = File::open(script)?;
    let mut remote_file = sftp.create(remote_script_path)?;
    std::io::copy(&mut script_file.take(usize::MAX as u64), &mut remote_file)?;

    /* script exec permission */
    let mut channel = session.channel_session()?;
    channel.exec(&format!("chmod +x {:?}", remote_script_path))?;
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;

    /* exec */
    let mut channel = session.channel_session()?;
    channel.exec(&format!("bash {:?}", remote_script_path))?;

    /* output */
    let mut output = Vec::new();
    channel.read_to_end(&mut output)?;
    let ret = String::from_utf8_lossy(&output);
    channel.wait_close()?;
    Ok(ret.to_string())
}
