Linux System Resource Monitor

Overview
Linux System Resource Monitor is a fault-tolerant monitoring tool designed to track CPU, memory, and disk usage with automated real-time alerts. The system provides process-level tracing, anomaly detection, and logging mechanisms to ensure accurate diagnostics and improved system reliability. Built using Rust for performance and efficiency, it supports concurrent monitoring of multiple metrics and integrates seamlessly with Docker for containerized deployment.

Features
- Real-time monitoring of CPU, memory, and disk usage
- Automated alerts when resource thresholds are exceeded
- Process-level tracing and detailed diagnostic logging
- Anomaly detection for identifying unusual patterns in system behavior
- Multi-threaded design for optimized performance under load
- Configurable thresholds and alerting mechanisms
- Dockerized for easy deployment and scalability
- CI/CD pipeline integration with GitHub Actions for continuous testing

Tech Stack
- Rust for core monitoring and performance optimization
- Bash for auxiliary scripting and automation
- gRPC for inter-process communication
- Docker for containerized deployment
- GitHub Actions for CI/CD workflows
- Linux as the operating environment

Installation
1. Clone the repository
   git clone https://github.com/Charanlokesh22/linux-system-resource-monitor.git
   cd linux-system-resource-monitor

2. Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Build the project
   cargo build --release

4. Run the application
   cargo run

5. To run inside Docker
   docker build -t linux-monitor .
   docker run -it linux-monitor

Usage
- By default, the monitor collects CPU, memory, and disk statistics every 5 seconds
- Logs are stored in the logs/ directory
- Threshold values can be configured in config/config.toml
- Alerts are printed to the console and written to alert.log

Example Output
CPU Usage: 35%
Memory Usage: 42%
Disk Usage: 57%
[ALERT] High CPU usage detected: 92%
[ALERT] Memory usage above threshold: 85%

Project Structure
src/
   main.rs          - Entry point for the monitor
   monitor.rs       - CPU, memory, and disk monitoring logic
   alerts.rs        - Threshold detection and alert handling
   logger.rs        - Logging utilities
config/
   config.toml      - Configurable threshold values
logs/
   alert.log        - Alerts recorded with timestamps
   system.log       - Continuous system monitoring logs
Dockerfile          - Containerization setup
.github/workflows/  - CI/CD pipeline definitions

Contributing
Contributions are welcome. Fork the repository, create a feature branch, and submit a pull request.

