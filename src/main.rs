use tokio_cron_scheduler::{JobScheduler, Job};
use std::{sync::Arc, clone};
use dotenv::dotenv;
use std::{env, fs};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();
    let crontab_path = env::var("CRONTAB").expect("Crontab not found");
    let crontab = read_crontab(&crontab_path);
    let mut sched = JobScheduler::new();
    for line in crontab{
        let (schedule_str, url) = get_data(&line);
        let async_job = Job::new(&schedule_str, move |_, _| {
            let clone_url: String = url.clone();
            tokio::spawn(async move{
                call(&clone_url).await;
            });
        }).unwrap();
        sched.add(async_job);
    };
    let _result = sched.start().await;
}

async fn call(url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url)
        .await?
        .status();
    println!("{}", response);
    Ok(response.to_string())
}

fn get_data(line: &str) -> (String, String){
    let parts: Vec<&str> = line.split(';').collect();
    let part1 = parts.get(0).unwrap();
    let part2 = parts.get(1).unwrap();
    println!("{} - {}", part1, part2);
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
