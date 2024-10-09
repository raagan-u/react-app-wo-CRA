use std::env;
use std::process::{Command, Output};

fn exec_command(command: &str, args: &[&str]) -> Result<Output, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("[x] failed to execute command {}", e))?;

    if output.status.success() {
        Ok(output)
    } else {
        let stderr= String::from_utf8_lossy(&output.stderr);
        Err(format!("[x] failed command output: {}", stderr))
    }
}

fn validate_output(result: Result<Output, String>, message: &str, verbose: bool) {
    println!("{}", message);

    match result {
        Ok(output) => {
            if verbose {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("The executed command output is \n{}", stdout);
            }
        }
        Err(error) => {
            println!("error occurred \n{}", error)
        }
    }
}

fn main() {

    println!("[*] script to create a react app from scratch...");
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&String::from("-v"));

    let commands = vec![
        (vec!["npm", "init", "-y"], "[*] initializing npm project."),
        (vec!["npm", "install", "react", "react-dom"], "[*] installing react & react-dom"),
        (vec!["npm", "install", "webpack", "--save-dev"], "[*] installing webpack"),
        (vec!["npm", "install", "webpack-cli", "webpack-dev-server", "--save-dev"], "[*] installing webpack cli & dev-server"),
        (vec!["npm", "install", "@babel/core", "@babel/preset-react", "@babel/preset-env", "babel-loader", "--save-dev"], "[*] installing babel & babel-components"),
        (vec!["npm", "install", "html-webpack-plugin", "--save-dev"], "[*] installing html-webpack-plugin"),
        (vec!["touch", "webpack.config.js"], "[*] creating webpack.config.js")
    ];

    for (command, message) in commands {
        validate_output(exec_command(command[0], &command[1..]), message, verbose);
    }
}
