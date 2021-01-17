# Jsonline Logger: Log Organizer

A tool for deserializing jsonline stdout and filtering the output based on arguments: 

- (1) log level (eg. info, warn, error, etc.)
- (2) fields (eg. appName, message, etc.)

Written in Rust using termion, serde_json, and clap. 

## Usage

The tool was created due to the need to go through a large log dump of jsonline data. Wanting to sort through the information by level, and wanting to only see the message fields of the logs, the tool lets a user specify the levels of a log they want to read, and the fields they would like to see. If no field is specified, the entire jsonline will be output.

Multiple parameters for --level and --fields are input via a delimited list, as seen below:

(Usage example)
```
kubectl logs -n <namespace> <pod name> | jl -l <level1>,<level2>,<level3> -f <field1>,<field2>,<field3>,<etc> | less -r
```

The output of jl can be piped into less (with the -r flag) to search and navigate the output instead of scrolling through the terminal.

## Installation

A linux binary has been uploaded in the Release section of this github page. Otherwise, the project used rust nightly to compile, and can be compiled via "cargo build --release" if you download the source code.

### Test Functionality

From inside the cloned repository run the following:
```
cat samples/log-sample.log | target/release/jl -l info,error -f level,message
```
