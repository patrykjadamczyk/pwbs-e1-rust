use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::Command;
use serde_json;
use std::env::args;

struct PwbsInfo {
    version: &'static str,
    edition: &'static str,
}

const PWBS: PwbsInfo = PwbsInfo {
    version: "0.9.1.0",
    edition: "E1 Rust",
};

fn baner() {
    println!("PAiP Web Build System {} Edition {}", PWBS.version, PWBS.edition)
}

struct PWBSConfigFile {
    commands : HashMap<String, Vec<String>>,
}

fn read_json_res(filename: String) -> Result<PWBSConfigFile, String> {
    let data = read_to_string(filename)
        .map_err(| err | {
            format!("Error in reading file: {}", err)
        })?;
    let mut json_data_object = PWBSConfigFile { commands: HashMap::new() };
    let json_data: serde_json::Value = serde_json::from_str(data.as_str())
        .map_err(|err: serde_json::Error| {
            format!("Error in parsing json: {}", err.to_string())
        })?;
    let j1 = json_data.as_object();
    if j1.is_none() {
        return Err("Invalid json".to_string());
    }
    let j1 = j1.unwrap();
    if !(j1.contains_key("commands")) {
        return Err("Invalid json".to_string());
    }
    let j2 = j1.get("commands").unwrap();
    if !(j2.is_object()) {
        return Err("Invalid json".to_string());
    }
    let commands = j2.as_object().unwrap();
    commands.iter().for_each(|(k, v): (&String, &serde_json::Value)| {
        if v.is_array() {
            let j3 = v.as_array().unwrap();
            let mut j4err = false;
            let j4: Vec<String> = j3.iter().map(| v: &serde_json::Value | {
                let vs = v.as_str();
                if vs.is_none() {
                    j4err = true;
                    "".to_string()
                } else {
                    vs.unwrap().to_string()
                }
            }).collect();
            json_data_object.commands.insert(k.to_string(), j4);
        } else if v.is_string() {
            let j3 = v.as_str().unwrap().to_string();
            json_data_object.commands.insert(k.to_string(), vec![j3]);
        } else {
        }
    });
    return Ok(json_data_object);
}

fn read_json(filename: String) -> PWBSConfigFile {
    match read_json_res(filename) {
        Ok(r) => r,
        Err(err) => {
            println!("{}", err);
            PWBSConfigFile{ commands: HashMap::new() }
        }
    }
}

fn execute(command: String, args: String) -> String {
    let output = Command::new(command.clone())
        .args(args.split(" "))
        .output()
        .expect(format!("Failed to execute command: {} {}", command, args).as_str());
    if !output.status.success() {
        return String::new();
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn pwbs_main(args: Vec<String>) {
    let json_data = read_json("pwbs.json".to_string());

    for arg in args {
        println!("Executing task \"{}\" ...", arg);
        if !json_data.commands.contains_key(arg.as_str()) {
            eprintln!("Task not found");
            continue;
        }
        let command_value = json_data.commands.get(arg.as_str()).unwrap();
        for command in command_value {
            let c: Vec<&str> = command.as_str().splitn(2, " ").collect();
            let (cmd, arguments) = (c[0].to_string(), c[1].to_string());
            let output = execute(cmd, arguments);
            println!("{}", output);
        }
        println!("Finished task \"{}\" ...", arg)
    }
}

fn main() {
    baner();
    let argv: Vec<String> = args().collect();
    let program_arguments: Vec<String> = argv[1..].to_vec();
    pwbs_main(program_arguments);
}
