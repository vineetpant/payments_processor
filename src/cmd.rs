use clap::{AppSettings, Arg, ArgMatches, Command};

pub fn get_argument_value<'a>(
    matches: &'a ArgMatches,
    arg_name: &'a str,
    fallback: Option<&'a str>,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    match matches.value_of(arg_name) {
        Some(value) => Ok(value),
        None => match fallback {
            Some(value) => Ok(value),
            None => Err(Box::from(format!("no value for {} given", arg_name))),
        },
    }
}

pub fn get_app<'a>() -> Result<Command<'a>, Box<dyn std::error::Error>> {
    Ok(Command::new("processor")
        .version("0.0.1")
        .author("Vineet Pant")
        .about("A simple toy payments engine that reads a series of transactions from a CSV.")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::new("file")
        .required(true)
        .help("CSV file path")
        .takes_value(true)
    )
    )
}