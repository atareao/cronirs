use tokio_cron_scheduler::{JobScheduler, Job};
use dotenv::dotenv;
use std::{env, fs};
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    println!("Starting croni");
    println!("==============");
    dotenv().ok();
    let crontab_path = env::var("CRONTAB").expect("Crontab not found");
    let crontab = read_crontab(&crontab_path);
    let mut sched = JobScheduler::new();
    for line in crontab{
        let (schedule_str, url) = get_data(&line);
        let async_job = Job::new(&schedule_str, move |_, _| {
            let clone_url: String = url.clone();
            tokio::spawn(async move{
                let _result = call(&clone_url).await;
            });
        }).unwrap();
        let _result = sched.add(async_job);
    };
    let _result = sched.start().await;
}

async fn call(url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url)
        .await?
        .status();
    let local: DateTime<Local> = Local::now();
    println!("{} | {} => {}", local.to_rfc3339(), url, response);
    Ok(response.to_string())
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
