use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, process::Command, io::{Write, Read}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Github {
    repo: String,
    branch: String,
    deploy_on_push: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Git {
    branch: String,
    repo_clone_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gitlab {
    repo: String,
    branch: String,
    deploy_on_push: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentVariable {
    key: String,
    value: String,
    scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Route {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Origin {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cors {
    allow_origins: Option<Vec<Origin>>,
    allow_methods: Option<Vec<String>>,
    allow_headers: Option<Vec<String>>,
    expose_headers: Option<Vec<String>>,
    max_age: Option<u32>,
    allow_credentials: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSpecService {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<Git>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<Github>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<AppSpecImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab: Option<Gitlab>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub envs: Option<Vec<EnvironmentVariable>>,
    pub instance_size_slug: String,
    pub instance_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<u32>,
    pub routes: Vec<Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<serde_yaml::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<serde_yaml::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ports: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<serde_yaml::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destinations: Option<Vec<serde_yaml::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSpecImage {
    registry_type: String,
    registry: Option<String>,
    repository: String,
    tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSpec {
    pub name: String,
    pub region: String,
    pub services: Vec<AppSpecService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<serde_yaml::Value>>,
    pub envs: Vec<EnvironmentVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<Vec<serde_yaml::Value>>,
    pub functions: Option<Vec<serde_yaml::Value>>,
    // image:AppSpecImage
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    id: String,
    owner_uuid: String,
    spec: AppSpec,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Operation {
    Update,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Context {
    App,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_enum)]
    pub context: Context,
    #[clap(value_enum)]
    pub operation: Operation,
    #[clap(value_parser)]
    pub app_id: String,
    #[clap(value_parser)]
    pub path: String,
    #[clap(value_parser)]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoctlError {
    detail: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoctlErrors {
    errors: Vec<DoctlError>,
}

impl std::fmt::Display for DoctlErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for doctl_error in self.errors.iter() {
            writeln!(f, "{}", doctl_error)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for DoctlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match (args.context, args.operation) {
        (Context::App, Operation::Update) => {
            // get a list of apps connected to this account
            println!("Getting a list of apps...");
            let apps_cmd = Command::new("doctl")
                .arg("apps")
                .arg("list")
                .args(&["--output", "json"])
                .output()?;
            // the command pipes this output to stdout. create a string slice
            let apps_cmd_stdout_str =
                std::str::from_utf8(&apps_cmd.stdout).expect("Could not parse output as utf-8");
            // the command also outputs errors as json
            let errors_json: Result<DoctlErrors, serde_json::Error> =
                serde_json::from_str(apps_cmd_stdout_str);
            if errors_json.is_ok() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    errors_json.unwrap().to_string(),
                ));
            }
            // run serde_json and serialize the json into a struct
            println!("Serializing the result...");
            let cmd_result_serialized: Vec<App> = serde_json::from_str(apps_cmd_stdout_str).unwrap();
            // find the app with the given app id
            let mut app = cmd_result_serialized.into_iter().find(|app| app.id == args.app_id).expect(&format!("No app with the app id {} was found", args.app_id)).to_owned();
            let path: Vec<&str> = args.path.split(".").collect();
            // enumerate the path to a given length, then match string slices and perform the update if supported
            match (path.get(0),path.get(1),path.get(2),path.get(3),path.get(4)) {
                (Some(&"spec"),Some(&"services"),Some(service_name),Some(&"image"),Some(&"tag")) => {
                    // find the service index for the service name
                    println!("Trying to update app.spec.services.{}.image.tag for app \"{}\"",*service_name,args.app_id);
                    let service_index = app.spec.services.iter().position(|service| service.name.as_str() == *service_name);
                    if service_index.is_none() {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("No service with the name '{}' was found",service_name)))
                    }
                    // get the service via the service index
                    let mut service = app.spec.services.get(service_index.unwrap()).unwrap().to_owned();
                    if service.image.is_none() {
                        // there isnt any support for writing this value yet as more data is needed
                        panic!("path 'app.spec.services.{}.image' was not present. Refusing to update",service_name)
                    }
                    // perform the update
                    let mut image = service.image.to_owned().unwrap();
                    image.tag = args.value;
                    service.image = Some(image);

                    app.spec.services[service_index.unwrap()] = service;
                    // convert app.spec into a yaml string
                    let app_spec_str = serde_yaml::to_string(&app.spec).expect("Failed to deserialize app.spec");
                    // create and spawn the validation command, piping the app_spec_str to stdin
                    let doctl_update_process = Command::new("doctl").arg("apps").arg("update").arg(app.id).arg("--wait").args(&["--spec","-"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();

                    // pipe the data to std_out
                    doctl_update_process.stdin.unwrap().write_all(app_spec_str.as_bytes())?;
                    let mut output: String = String::new();
                    doctl_update_process.stdout.unwrap().read_to_string(&mut output)?;
                    println!("{:#?}",output);

                },
                _=> panic!("Updating the path \"{}\" is not supported", args.path)
            }


        }
       rest @ _  => panic!("Unsupported Arguments {:#?}", rest),
    }

    Ok(())
}
