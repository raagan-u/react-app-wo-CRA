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

fn validate_output(result: Result<Output, String>) {
    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("The executed command output is \n{}", stdout);
        }
        Err(error) => {
            println!("error occurred \n{}", error)
        }
    }
}

fn main() {
    println!("[*] script to create a react app from scratch...");

    validate_output(exec_command("ls", &["-lh"]));
    
}
