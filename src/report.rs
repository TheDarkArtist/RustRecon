// src/report.rs

use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn generate_report(report_data: Vec<(u16, &'static str, Vec<&'static str>)>) {
    let report: Vec<_> = report_data
        .iter()
        .map(|(port, service, vulnerabilities)| {
            json!({
                "port": port,
                "service": service,
                "vulnerabilities": vulnerabilities,
            })
        })
        .collect();

    let mut file = File::create("report.json").expect("Failed to create report file");
    file.write_all(
        serde_json::to_string_pretty(&report)
            .expect("Failed to serialize report data")
            .as_bytes(),
    )
    .expect("Failed to write report file");
}

