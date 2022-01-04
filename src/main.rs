use clap::Parser;
use std::process::Command;

fn exec_command(command: &str) -> String {
    let output = Command::new("sh")
                          .args(["-c", command])
                          .output()
                          .expect("git not found");

    let result_buff = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    match String::from_utf8(result_buff) {
        Ok(v) => v,
        Err(_) => panic!("error parsing")
    }
}

fn get_remote_url() -> String {
    exec_command("git remote get-url --push origin")
}

#[derive(Parser)]
#[clap(about)]
struct Args {

    name: String,

}


fn main() {
    let args = Args::parse();

    let name: String = match args.name.as_str() {

        "url" => get_remote_url(),
        _     => "go to dick".to_string()

    };

    println!("{}", name);
}

