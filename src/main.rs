use tokio_cron_scheduler::{JobScheduler, Job};
use std::{env, fs, collections::VecDeque, str::FromStr};
use tokio::process::Command;
use simplelog::{SimpleLogger, Config, LevelFilter};
use log::{info, debug};

#[tokio::main]
async fn main() {
    let log_level: LevelFilter = LevelFilter::from_str(&env::var("LOG_LEVEL")
        .unwrap_or("info".to_string())).unwrap_or(LevelFilter::Info);
    let _ = SimpleLogger::init(log_level, Config::default());
    info!("Starting croni");
    info!("==============");
    info!("Log level: {}", &log_level);
    let crontab_path = env::var("CRONTAB").expect("Crontab not found");
    info!("Crontab: {}", &crontab_path);
    let crontab = read_crontab(&crontab_path);
    let mut sched = JobScheduler::new();
    for line in crontab{
        let (schedule_str, command) = get_data(&line);
        let async_job = Job::new(&schedule_str, move |_, _| {
            let command_clone: String = command.clone();
            tokio::spawn(async move{
                let result = call(&command_clone).await;
                debug!("{}", result);
            });
        }).unwrap();
        let _result = sched.add(async_job);
    };
    let _result = sched.start().await;
}

async fn call(command_line: &str) -> String{
    debug!("{}", &command_line);
    let mut cwa: VecDeque<&str> = command_line.split(" ").collect();
    let command = cwa.pop_front().unwrap();
    debug!("{}", &command);
    debug!("{:?}", &cwa);
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
