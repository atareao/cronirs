mod error;

use chrono::Local;
use std::{
    collections::VecDeque,
    env,
    fs,
};
use tokio::{
    process::Command,
    time::{sleep, Duration},
};
use tokio_cron::{Job, Scheduler};
use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use error::Error;

#[tokio::main]
async fn main() {
    let time_offset =
        time::UtcOffset::from_whole_seconds(Local::now().offset().local_minus_utc()).unwrap();
    let timer_format = time::format_description::well_known::Iso8601::DEFAULT;
    let log_layer = tracing_subscriber::fmt::layer()
        .with_timer(fmt::time::OffsetTime::new(time_offset, timer_format))
        .compact()
        .with_level(true)
        .with_thread_names(true)
        .with_filter(EnvFilter::from_env("LOG_LEVEL"));
    tracing_subscriber::registry().with(log_layer).init();
    info!("Starting croni");
    info!("==============");
    info!("Log level: {}", EnvFilter::from_env("LOG_LEVEL"));
    let crontab_path = env::var("CRONTAB").expect("Crontab not found");
    info!("Crontab: {}", &crontab_path);
    let crontab = read_crontab(&crontab_path);
    let mut sched = Scheduler::local();
    for line in crontab {
        if !line.is_empty() && line.contains(';'){
            match get_data(&line){
                Ok((schedule, command)) => {
                    sched.add(Job::new(
                        schedule.as_str(),
                        move || call(command.clone())));
                },
                Err(e) => panic!("Error: {e}"),
            }
        }else if !line.is_empty(){
            panic!("command line is wrong: {line}")
        }
    }
    info!("Start ok");
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

async fn call(command_line: String) {
    debug!("{}", &command_line);
    let mut cwa: VecDeque<&str> = command_line.split(' ').collect();
    let command = cwa.pop_front().unwrap();
    debug!("{}", &command);
    debug!("{:?}", &cwa);
    let output = Command::new(command)
        .args(cwa)
        .output()
        .await
        .unwrap_or_else(|_| panic!("Failed to run {}", &command));
    let output = format!(
        "Status: {}\nOutput: {}\nError: {}\n",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    debug!("Output: {}", output);
}

fn get_data(line: &str) -> Result<(String, String), Error> {
    let parts: Vec<&str> = line.split(';').collect();
    let part1 = parts.first()
        .ok_or(Error::new("First part no found"))?;
    let part2 = parts.last()
        .ok_or(Error::new("First part no found"))?;
    Ok((part1.to_string(), part2.to_string()))
}
fn read_crontab(path: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        lines.push(line.to_string());
    }
    lines
}
