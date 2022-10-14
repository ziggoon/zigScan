extern crate ftp;

use colored::Colorize;
use ftp::types::FileType;
use ftp::FtpStream;
use std::fs::File;
use std::process::{Command, Stdio};

pub fn banner() {
    let banner = r#"                                                             
@@@@@@@@  @@@   @@@@@@@@   @@@@@@    @@@@@@@   @@@@@@   @@@  @@@  
@@@@@@@@  @@@  @@@@@@@@@  @@@@@@@   @@@@@@@@  @@@@@@@@  @@@@ @@@  
     @@!  @@!  !@@        !@@       !@@       @@!  @@@  @@!@!@@@  
    !@!   !@!  !@!        !@!       !@!       !@!  @!@  !@!!@!@!  
   @!!    !!@  !@! @!@!@  !!@@!!    !@!       @!@!@!@!  @!@ !!@!  
  !!!     !!!  !!! !!@!!   !!@!!!   !!!       !!!@!!!!  !@!  !!!  
 !!:      !!:  :!!   !!:       !:!  :!!       !!:  !!!  !!:  !!!  
:!:       :!:  :!:   !::      !:!   :!:       :!:  !:!  :!:  !:!  
 :: ::::   ::   ::: ::::  :::: ::    ::: :::  ::   :::   ::   ::  
: :: : :  :     :: :: :   :: : :     :: :: :   :   : :  ::    :  
"#;
    print!("{}", banner.red());
}

// using rustscan we can quickly run multiple network scans and push to the backend
pub fn basic_tcp(target: &str) {
    println!("{}", "[+] starting {{ PING }} rustscan...".yellow());
    let scan_child = Command::new("rustscan")
        .args(["-a", target, "--", "-oX", "scans/basic_tcp.xml"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("initial nmap scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("scans/basic_tcp.xml");
}

pub fn full_tcp(target: &str) {
    println!("{}", "[+] starting {{ FULL TCP }} rustscan...".yellow());
    let scan_child = Command::new("rustscan")
        .args([
            "-a",
            target,
            "-r",
            "1-65535",
            "--",
            "-sS",
            "-T4",
            "-oX",
            "scans/full_tcp.xml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("full tcp scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("scans/full_tcp.xml")
}

pub fn intense_scan(target: &str) {
    println!("{}", "[+] starting {{ INTENSE }} rustscan...".yellow());
    let scan_child = Command::new("rustscan")
        .args([
            "-a",
            target,
            "-r",
            "1-65535",
            "--",
            "-sS",
            "-sV",
            "-A",
            "-T4",
            "-oX",
            "scans/intense.xml",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("intense scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("intense.xml");
}

pub fn basic_udp(target: &str) {
    println!("{}", "[+] starting {{ BASIC UDP }} rustscan...".yellow());
    let scan_child = Command::new("rustscan")
        .args(["-a", target, "--", "-sU", "-oX", "scans/basic_udp.xml"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("basic udp scan failed");

    let _output = scan_child.wait_with_output().expect("scan is fucked");
    post_ftp("scans/basic_udp.xml")
}

fn post_ftp(filename: &str) {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").expect("Connection error");
    let _ = ftp_stream.login("user", "pass").unwrap();

    ftp_stream.transfer_type(FileType::Binary);
    let mut f1 = File::open(filename).unwrap();

    let _ = ftp_stream.put(filename, &mut f1);
    let _ = ftp_stream.quit();
    println!("{}", "  [-] file uploaded".green())
}
