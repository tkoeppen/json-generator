use clap::{Arg, ArgMatches, Command};
use json_gen::generate;
use json_gen::generator::generators::read_file_into_string;
use json_gen::json_template::JsonTemplate;
use json_gen::sender::file::{FileSender, FolderSender};
use json_gen::sender::http::CurlSender;
use json_gen::sender::{ConsoleSender, Sender};
use serde_json::Value;
use simplelog::*;

#[macro_use]
pub extern crate log;

fn main() {
    let args = get_args();
    if args.get_one::<bool>("logs").is_some() {
        SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap()
    }

    generate_from_args(&args);
}

fn create_args() -> Command {
    Command::new("json-gen")
        .version("0.2.3")
        .author("Boris Zhguchev <zhguchev@gmail.com>")
        .about("The json generator with ability to generate dynamic fields.")
        .arg(
            Arg::new("jt-file")
                .short('f')
                .long("file")
                .allow_hyphen_values(true)
                .conflicts_with("jt-body")
                .help("the file containing the json template"))
        .arg(
            Arg::new("jt-body")
                .short('b')
                .long("body")
                .allow_hyphen_values(true)
                .conflicts_with("jt-file")
                .help("the text representation containing the json template"))
        .arg(
            Arg::new("repeater")
                .short('r')
                .long("repeat")
                .help("how many repetition needs to perform"))
        .arg(
            Arg::new("indicator")
                .short('i')
                .long("indicator")
                .help("the prefix signalling the field contains a generator"))
        .arg(
            Arg::new("to-curl")
                .long("to-curl")
                .allow_hyphen_values(true)
                .help("to send the request through the curl utility using this param and adding json body (curl utility needs to be installed)"))
        .arg(
            Arg::new("to-folder")
                .long("to-folder")
                .allow_hyphen_values(true)
                .help("to save the generated jsons in the folder"))
        .arg(
            Arg::new("to-file")
                .long("to-file")
                .allow_hyphen_values(true)
                .help("save the generated jsons to the file"))
        .arg(
            Arg::new("to-console")
                .long("to-console")
                .number_of_values(0)
                .help("to display the generated jsons in the console(by default if outputs array is empty)"))
        .arg(
            Arg::new("pretty-js")
                .long("pretty")
                .number_of_values(0)
                .help("to format the generated json into the readable view"))
        .arg(
            Arg::new("logs")
                .long("logs")
                .number_of_values(0)
                .help("to print extra logs"))
}

fn get_args() -> ArgMatches {
    create_args().get_matches()
}

fn validate_repeat(args: &ArgMatches) -> u32 {
    let repeat_str = args.get_one::<String>("repeater");
    match repeat_str {
        Some(s) => {
            let repeat = s.parse::<u32>();
            match repeat {
                Ok(r) => r,
                Err(e) => {
                    error!("the repetition number should be a positive integer, greater than zero");
                    panic!("{}", e)
                }
            }
        }
        None => 1,
    }
}

fn output(args: &ArgMatches) -> Vec<Box<dyn Sender>> {
    let mut outputs: Vec<Box<dyn Sender>> = vec![];
    if let Some(str) = args.get_one::<String>("to-file") {
        debug!("new output to the file: {}", str);
        outputs.push(Box::new(FileSender::new(str.to_string())))
    }
    if let Some(str) = args.get_one::<String>("to-folder") {
        debug!("new output to the folder: {}", str);
        outputs.push(Box::new(FolderSender::new(str.to_string())))
    }
    if let Some(str) = args.get_one::<String>("to-curl") {
        debug!("new output to the server: {}", str);
        outputs.push(Box::new(CurlSender::new(str.to_string())))
    }
    if args.get_one::<bool>("to-console").is_some() {
        debug!("new output to the console");
        outputs.push(Box::new(ConsoleSender {}))
    }
    if outputs.is_empty() {
        debug!("set the output to the console");
        outputs.push(Box::new(ConsoleSender {}))
    }
    outputs
}

fn json_template(args: &ArgMatches) -> JsonTemplate {
    debug!("try to parse the json template...");
    let txt = match (
        args.get_one::<String>("jt-body"),
        args.get_one::<String>("jt-file"),
    ) {
        (Some(body), _) => {
            debug!("ready to obtain the json template from the body {}", body);
            String::from(body)
        }
        (None, Some(file)) => {
            debug!("ready to obtain the json template from the file {}", file);
            read_file_into_string(file).expect("exception with the processing the file!")
        }
        (None, None) => {
            panic!("the input file or body containing the json template should be provided!")
        }
    };
    let binding = "|".to_owned();
    let indicator = args.get_one::<String>("indicator").unwrap_or(&binding);
    debug!("the json template with indicator[{}] {}", indicator, txt);
    match JsonTemplate::from_str(txt.as_str(), indicator) {
        Ok(t) => t,
        Err(e) => panic!("error while parsing json : {:?}", e),
    }
}

fn generate_from_args(args: &ArgMatches) -> Vec<Value> {
    generate(
        &mut json_template(args),
        validate_repeat(args),
        args.get_flag("pretty-js"),
        &mut output(args),
    )
}

#[cfg(test)]
mod tests {
    use crate::{create_args, generate_from_args};

    #[test]
    fn find_json_text() {
        let jt_body = r#"{"|id": "uuid()"}"#;
        let args = create_args().get_matches_from(vec![
            "",
            format!("--body={}", jt_body).as_str(),
            "--pretty",
        ]);
        let res = generate_from_args(&args);
        assert_eq!(res.len(), 1);
        assert_eq!(
            res.get(0)
                .and_then(|v| v.as_object())
                .unwrap()
                .get("id")
                .and_then(|e| e.as_str())
                .unwrap()
                .len(),
            36
        );
    }
}
