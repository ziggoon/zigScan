extern crate ftp;

use ftp::types::FileType;
use ftp::FtpStream;
use std::fs::File;
use std::process::{Command, Stdio};

pub fn banner() {
    let banner = r#"
      _____                   ____            
     / ___/______  __ _____  / __/______ ____ 
    / (_ / __/ _ \/ // / _ \_\ \/ __/ _ `/ _ \
    \___/_/  \___/\_,_/ .__/___/\__/\_,_/_//_/
                     /_/                      
"#;
    print!("{}", banner);
}

// using rustscan we can quickly run multiple network scans and push to the backend
pub fn basic_scan(target: &str) {
    println!("[+] starting initial rustscan...");
    let scan_child = Command::new("rustscan")
        .args(["-a", target, "--", "-oX", "scans/init.xml"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("initial nmap scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("scans/init.xml");
}

pub fn secondary_scan(target: &str) {
    println!("[+] starting secondary rustscan...");
    let scan_child = Command::new("rustscan")
        .args([
            "-a",
            target,
            "--",
            "-sV",
            "-A",
            "-T4",
            "-oX",
            "scans/secondary.xml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("secondary nmap scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("scans/secondary.xml");
}

pub fn post_ftp(filename: &str) {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp_stream.login("user", "pass").unwrap();

    ftp_stream.transfer_type(FileType::Binary);

    let mut f1 = File::open(filename).unwrap();

    let _ = ftp_stream.put(filename, &mut f1);
    let _ = ftp_stream.quit();
}
