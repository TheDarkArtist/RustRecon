// src/main.rs

mod config;
mod report;
mod scanner;

use config::Config;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Load configuration from config.toml
    let config = Config::from_file("config.toml");

    // Setup communication channel for port results
    let (tx, mut rx) = mpsc::channel::<u16>(4);

    // Spawn tasks to scan each port asynchronously
    for port in &config.ports {
        let tx = tx.clone();
        let ip_addr = format!("{}:{}", config.ip, port)
            .parse()
            .expect("Invalid IP address format");

        tokio::spawn(async move {
            scanner::scan_port(ip_addr, tx).await;
        });
    }

    drop(tx); // Drop sender to terminate the channel

    let mut report_data = vec![];

    // Receive port results and generate report
    while let Some(port) = rx.recv().await {
        let service = match port {
            22 => "SSH",
            80 => "HTTP",
            443 => "HTTPS",
            8080 => "HTTP",
            _ => "Unknown",
        };

        let vulnerabilities = match service {
            "SSH" => vec!["CVE-2020-14001", "CVE-2018-15473"],
            "HTTP" => vec!["CVE-2019-12345", "CVE-2017-5638"],
            "HTTPS" => vec!["CVE-2020-12345"],
            _ => vec![],
        };

        report_data.push((port, service, vulnerabilities));
    }

    // Generate JSON report
    report::generate_report(report_data);
}
