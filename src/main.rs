use std::fs::{File, create_dir_all, read_to_string, write};
use std::io::Write;
use std::env;
use std::process::{Command, Output};
use serde_json::{Value, from_str};

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

fn create_file(file_name: &str, content: &str) -> Result<(), String> {
    let mut file = File::create(file_name).map_err(|e| e.to_string())?; // Create the file
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?; // Write content
    Ok(())
}

fn package_json_modifier() -> Result<(), Box<dyn std::error::Error>> {
    let contents  = read_to_string("package.json")?;
    let mut vals:Value = from_str(&contents).unwrap();

    
    vals["scripts"]["build"] = Value::String("webpack --mode production".to_string());
    vals["scripts"]["start"] = Value::String("webpack-dev-server --mode development --open --hot".to_string());
    
    let updated_contents = serde_json::to_string_pretty(&vals)?;
    
    //println!("Updated package.json:\n{}", updated_contents);
    
    write("package.json", updated_contents)?;
    Ok(())
 }



fn main() {

    println!("********* script to create a react app from scratch. *********");

    let args: Vec<String> = env::args().collect();
    let verbose: bool = args.contains(&String::from("-v"));

    if let Err(e) = create_dir_all("react-app") {
        eprintln!("Failed to create public directory: {}", e);
        return;
    }

    println!("[+] created \"react-app\" directory.");

    std::env::set_current_dir("react-app").expect("Failed to change directory");

    let commands = vec![
        (vec!["npm", "init", "-y"], "[*] initializing npm project."),
        (vec!["npm", "install", "react", "react-dom"], "[*] installing react & react-dom"),
        (vec!["npm", "install", "webpack", "--save-dev"], "[*] installing webpack"),
        (vec!["npm", "install", "webpack-cli", "webpack-dev-server", "--save-dev"], "[*] installing webpack cli & dev-server"),
        (vec!["npm", "install", "@babel/core", "@babel/preset-react", "@babel/preset-env", "babel-loader", "--save-dev"], "[*] installing babel & babel-components"),
        (vec!["npm", "install", "html-webpack-plugin", "--save-dev"], "[*] installing html-webpack-plugin")
    ];

    for (command, message) in commands {
        validate_output(exec_command(command[0], &command[1..]), message, verbose);
    }

    if let Err(e) = create_dir_all("public") {
        eprintln!("Failed to create public directory: {}", e);
        return;
    }

    if let Err(e) = create_dir_all("src") {
        eprintln!("Failed to create src directory: {}", e);
        return;
    }
    
    if let Err(e) = create_dir_all("./src/components") {
        eprintln!("Failed to create src directory: {}", e);
        return;
    }

    let files_to_create = vec![
        ("webpack.config.js", r#"
const path = require('path')
const HTMLWebpackPlugin = require('html-webpack-plugin')

module.exports={
    entry: "./src/index.js",
    output: {
        path: path.join(__dirname,"/dist"),
        filename: "bundle.js"
    },
    plugins: [
        new HTMLWebpackPlugin({
            template: "./public/index.html"
        })
    ],
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: ["@babel/preset-react", "@babel/preset-env"]
                    }
                }
            }
        ]
    },
    
} 
"#),
        ("./src/index.js", r#"
import React from "react";
import ReactDOM from "react-dom";
import App from "./components/App";

ReactDOM.render(<App/> , document.getElementById('root'))
"#),
("./src/components/App.js", r#"
import React , {Component} from "react";

class App extends Component {
    render(){
        return(
            <div>
                Hello World!
            </div>        
        )
    }
}
export default App;
"#),
        ("./public/index.html", r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Document</title>
    </head>
    <body>
        <div id="root"></div>
    </body>
</html>
"#)
    ];

    // Create the specified files with their contents
    for (file_name, content) in files_to_create {
        match create_file(file_name, content) {
            Ok(_) => {
                if verbose {
                    println!("Created file: {}", file_name);
                }
            },
            Err(e) => {
                eprintln!("Failed to create file {}: {}", file_name, e);
                return; // Exit if there's an error
            }
        }
    }
    package_json_modifier().unwrap()
    println!("change into \"react-app\" directory and type \"npm run start\" to start react app!!")
}
