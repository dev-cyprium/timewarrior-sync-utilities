use std::{
    env,
    io::{self, BufReader, Cursor},
};

use colored::Colorize;
use ftp::FtpStream;
use inquire::{InquireError, Text};
use log::info;
use sync_time::{
    config::{
        config_file_path, config_file_present, create_config_file, fill_config_file, UserConfig,
    },
    relative_to_home,
};

fn upload_timewarrior_data(ftp_stream: &mut FtpStream) {
    // TODO: connection works! We can upload the data now
    // Make the server sync-up the files instead of choosing upload/download
    println!("uploading timewarrior data...");
    // build a absolute path from pwd() and the given path
    let home = env::var("HOME").expect("HOME not set");
    let path = format!("{}/.timewarrior/data", home);
    let _reader = BufReader::new(std::fs::File::open(path).unwrap());

    let _path_base = "/sda1/timewar";
    let r = ftp_stream.list(Some("/sda1/timewar")).unwrap();

    let mut c = Cursor::new("hello world".as_bytes());
    print!("r: {:?}", r);
    ftp_stream.put("/sda1/timewar/blah.data", &mut c).unwrap();
}

fn handle_config_file() {}

fn interactive_create_config() -> Result<(), InquireError> {
    let default_path = relative_to_home(".timewarrior-sync/config.toml");

    let config_path = Text::new("Where do you want to save your config file?")
        .with_default(&default_path)
        .prompt()?;

    info!("Saving config file to {}", config_path);
    let mut file = match create_config_file(&config_path) {
        Ok(f) => {
            info!("Config file created at {}", config_path);
            f
        }
        Err(err) => panic!("{}", err),
    };

    let msg = "Let's connect you to your FTP server!".blue();
    println!("{}", msg);

    let mut config = UserConfig::new();
    let questions: [(&str, &dyn Fn(&mut UserConfig, &str)); 4] = [
        (
            "What is the hostname of your FTP server? ",
            &|config, input| {
                config.set_hostname(input.to_string());
            },
        ),
        (
            "What is the port of your FTP server? ",
            &|config, input| loop {
                let port: u16 = input.parse().unwrap_or_else(|_| {
                    let msg = "Please enter a valid port number".bright_red();
                    println!("{}", msg);
                    let input = Text::new("What is the port of your FTP server? ")
                        .with_default("21")
                        .prompt()
                        .unwrap();
                    input.parse().unwrap()
                });

                config.set_port(port);
                break;
            },
        ),
        (
            "What is the username of your FTP server? ",
            &|config, input| {
                config.set_username(input.to_string());
            },
        ),
        (
            "What is the password of your FTP server? ",
            &|config, input| {
                config.set_password(input.to_string());
            },
        ),
    ];

    let mut answers = vec![];

    for (question, cb) in questions {
        let answer = Text::new(question).prompt()?;
        cb(&mut config, &answer);
        answers.push(answer);
    }

    fill_config_file(&mut file, &config)?;
    Ok(())
}

fn main() -> io::Result<()> {
    env_logger::init();
    info!("Booting up");

    info!("Checking for timewarrior presence");
    if !sync_time::is_program_in_path("timew") {
        let red_text = "Timewarrior not present.".bright_red();
        println!(
            "{} Please install timewarrior https://timewarrior.net/.",
            red_text
        );
        std::process::exit(1);
    }

    info!("Checking for config file presence");
    if !config_file_present() {
        info!("Config file not present, making one with user");
        let red_text = "Config file not present.".bright_red();
        println!("{} I can create one for you!", red_text);
        interactive_create_config().expect("Failed to create config file");
    }

    info!("Config file present, reading it");
    let config = UserConfig::from_file(config_file_path())?;

    if !config.validate() {
        let red_text = "Config file is invalid.".bright_red();
        println!("{} Please fix it.", red_text);
        println!(
            r#"
Example config file:
--------------------
    hostname = "myftp.server.com"
    port = 21
    username = "t"
    password = "t"
            "#
        );
        std::process::exit(1);
    }

    info!("Config file is valid, connecting to FTP server");
    let (hostname, port, username, password) = config.unwrap();

    let mut ftp_stream = FtpStream::connect(format!("{}:{}", hostname, port))
        .unwrap_or_else(|err| panic!("{}", err));
    ftp_stream
        .login(&username, &password)
        .expect("Login to FTP failed, please check your credentials");

    println!("Select an option: ");
    println!("1. Upload timewarrior data");
    println!("2. Download timewarrior data");
    println!("3. Exit");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    match input {
        1 => upload_timewarrior_data(&mut ftp_stream),
        // 2 => download_timewarrior_data(&mut ftp_stream, &username, &password),
        _ => println!("Invalid option"),
    }

    let _ = ftp_stream.quit();

    Ok(())
}
