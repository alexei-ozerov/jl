use clap::clap_app;
use serde_json::Value;
use std::io::BufRead;
use termion::color;

// Check STDIN for JSON Input, Append Into Vector, Return To MAIN
fn deserialize_stdin() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut log_vec: Vec<String> = Vec::new();

    for line in stdin.lines() {
        let l_res = line.unwrap();
        log_vec.push(l_res.to_string().clone());
    }

    Ok(log_vec)
}

fn main() {
    let matches = clap_app!(jl =>
        (version: "0.3")
        (author: "Alexei Ozerov. <alexei.ozerov.7@gmail.com>")
        (about: "Jsonline Logger.\n\nRecommended usage: kubectl logs -n <namespace> <pod name> | jl -l <level> -f <field1,field2,field3,etc> | less -r")
        (@arg LEVEL: -l --level +required +takes_value "Log Level (eg. -l info)")
        (@arg FIELDS: -f --fields +takes_value "Log Filters: Comma Delimited (eg. -f field1,field2,field3")
    )
    .get_matches();

    // Deserialize JSON from STDIN
    let res = deserialize_stdin();
    let res = match res {
        Ok(log_vec) => log_vec,
        Err(error) => panic!("FATAL ERROR: {}", error),
    };

    // Determine Output
    // TODO: Break up common code into functions ...
    for s in &res {
        if &s[..1] == "{" {
            let v: Value = serde_json::from_str(&s).unwrap();
            if v["level"] == matches.value_of("LEVEL").unwrap() {
                if let Some(f) = matches.value_of("FIELDS") {
                    let comma_delimit = f.split(",");
                    let fields_vec: Vec<&str> = comma_delimit.collect();
                    println!("\n");
                    for e in fields_vec {
                        println!(
                            "{}{}:{} {},",
                            color::Fg(color::Red),
                            e,
                            color::Fg(color::Reset),
                            v[e],
                        );
                    }
                    println!(
                        "{}timestamp:{} {}",
                        color::Fg(color::Red),
                        color::Fg(color::Reset),
                        v["timestamp"]
                    );
                } else {
                    println!("\n{}", v);
                }
            }
        } else {
            if s.contains("{\"") {
                let split = s.split("{\"");
                let split_vec: Vec<&str> = split.collect();
                for n in 0..split_vec.len() {
                    if split_vec[n].contains("\"}") {
                        let json_line = "{\"".to_owned() + &split_vec[n];
                        let v: Value = serde_json::from_str(&json_line).unwrap();
                        if v["level"] == matches.value_of("LEVEL").unwrap() {
                            if let Some(f) = matches.value_of("FIELDS") {
                                let comma_delimit = f.split(",");
                                let fields_vec: Vec<&str> = comma_delimit.collect();
                                println!("\n");
                                for e in fields_vec {
                                    println!(
                                        "{}{}:{} {},",
                                        color::Fg(color::Red),
                                        e,
                                        color::Fg(color::Reset),
                                        v[e],
                                    );
                                }
                                println!(
                                    "{}timestamp:{} {}",
                                    color::Fg(color::Red),
                                    color::Fg(color::Reset),
                                    v["timestamp"]
                                );
                            } else {
                                println!("\n{}", v);
                            }
                        }
                    } else {
                        println!(
                            "\n{}Warning, non-JSON Log Found:{}\n{}",
                            color::Fg(color::Yellow),
                            color::Fg(color::Reset),
                            split_vec[n]
                        );
                    }
                }
            } else {
                println!(
                    "\n{}Warning, non-JSON Log Found:{}\n{}",
                    color::Fg(color::Yellow),
                    color::Fg(color::Reset),
                    &s
                );
            }
        }
    }
}
