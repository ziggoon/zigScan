# zigScan

zigScan is a port scan visualizer built on [WebMap](https://github.com/SabyasachiRana/WebMap) with a Rust client to perform port scans using [Rustscan](https://github.com/RustScan/RustScan).

Nmap scans for penetration tests and offensive security competitions, like CPTC, can take extremely long and are often hard to parse by hand or distribute to coworkers. This tool was built with collaboration and speed in mind, which is why all port scanning is handled by Rustscan, a super speedy Rust client built on top of Nmap. 

# Usage

1. sudo apt-get update
2. sudo apt-get install -y docker.io
3. ./zigScan/webmap/docker/build.sh
       - use ACCESS_KEY from build output to access frontend @ 0.0.0.0:8000
5. ./zigScan/scanner/client
       - scan <target\>

## Contributors
[caret](https://github.com/641i130)

## License
Code is licensed unded GPT 3.0
