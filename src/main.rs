use clap::{Parser, Subcommand, ArgEnum};
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
struct Cli {

    #[clap(subcommand)]
    command: Command_,

}

#[derive(Subcommand)]
#[clap(about)]
enum Command_ {
    Get {
        #[clap(arg_enum)]
        resource: Resource,
    }
}

#[derive(Copy, Clone, ArgEnum)]
enum Resource {
    Url,
    Pipe
}


fn main() {
    let cli = Cli::parse();
    let name = match cli.command {

        Command_::Get{ resource: Resource::Url }    => get_remote_url(),
        Command_::Get{ resource: Resource::Pipe }   => todo!(),

    };


    println!("{}", name);
}

