# Raspberry Pi network ping tool in Rust

## Description

Generic network pinging tool, built in Rust, to periodically run ping and capture the output in a file. This is meant to passively capture network health on a Unix OS, such as Raspberry Pi OS.

## Usage

### Build
`cargo build`

### Help
`pi-pinger --help`

### Run
Default constant ping
`pi-pinger --ip <Target IP> constant`

Random ping
`pi-pinger --ip <Target IP> random --max 3600 --min 300`

### Output
Output is stored in a file in the following format: `<timestamp> <average ping>`