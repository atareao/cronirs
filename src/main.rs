use tokio_cron_scheduler::{JobScheduler, Job};
use dotenv::dotenv;
use std::{env, fs, collections::VecDeque, process::Stdio};
use chrono::prelude::*;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    println!("Starting croni");
    println!("==============");
    dotenv().ok();
    let crontab_path = env::var("CRONTAB").expect("Crontab not found");
    let crontab = read_crontab(&crontab_path);
    let mut sched = JobScheduler::new();
    for line in crontab{
        let (schedule_str, command) = get_data(&line);
        let async_job = Job::new(&schedule_str, move |_, _| {
            let command_clone: String = command.clone();
            tokio::spawn(async move{
                let result = call(&command_clone).await;
                println!("{}", result);
            });
        }).unwrap();
        let _result = sched.add(async_job);
    };
    let _result = sched.start().await;
}

async fn call(command_line: &str) -> String{
    println!("{}", &command_line);
    let mut cwa: VecDeque<&str> = command_line.split(" ").collect();
    let command = cwa.pop_front().unwrap();
    println!("{}", &command);
    println!("{:?}", &cwa);
    let output = Command::new(&command)
        .args(cwa)
        .output()
        .await
        .expect(&format!("Failed to run {}", &command));
    format!("Status: {}\nOutput: {}\nError: {}\n",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn get_data(line: &str) -> (String, String){
    let parts: Vec<&str> = line.split(';').collect();
    let part1 = parts.get(0).unwrap();
    let part2 = parts.get(1).unwrap();
    (part1.to_string(), part2.to_string())
}
fn read_crontab(path: &str) -> Vec<String>{
    let mut lines: Vec<String> = Vec::new();
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines(){
        lines.push(line.to_string());
    }
    lines
}
