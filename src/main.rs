use clap::clap_app;
use clap::ArgMatches;
use serde_json::Value;
use std::io::BufRead;
use termion::color;

// Append STDIN Into Vector, Return To MAIN
fn save_stdin() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let mut log_vec: Vec<String> = Vec::new();

    for line in stdin.lines() {
        let l_res = line.unwrap();
        log_vec.push(l_res.to_string().clone());
    }

    Ok(log_vec)
}

// Deserialize Jsonline String
fn deserialize_jsonline(json_string: &str, matches: &ArgMatches<'_>) {
    let v: Value = serde_json::from_str(&json_string).unwrap();
    
    // Get Info Vec
    let mut _level_vec: Vec<&str> = Vec::new();
    if let Some(l) = matches.value_of("LEVEL") {
        let level_delimit = l.split(",");
        _level_vec = level_delimit.collect();
    }

    // Filter Data & Print Results
    if _level_vec.contains(&v["level"].as_str().unwrap()) {
        if let Some(f) = matches.value_of("FIELDS") {
            let comma_delimit = f.split(",");
            let fields_vec: Vec<&str> = comma_delimit.collect();
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
                "{}timestamp:{} {}\n",
                color::Fg(color::Red),
                color::Fg(color::Reset),
                v["timestamp"]
            );
        } else {
            println!("\n{}", v);
        }
    }
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

    // Get Vec From STDIN
    let res = save_stdin();
    let res = match res {
        Ok(log_vec) => log_vec,
        Err(error) => panic!("FATAL ERROR: {}", error),
    };

    // Determine Output
    println!("\n");
    for s in &res {
        // if jsonline
        if &s[..1] == "{" {
            deserialize_jsonline(&s, &matches);
        } else {
            // if contains jsonline
            if s.contains("{\"") {
                let split = s.split("{\"");
                let split_vec: Vec<&str> = split.collect();
                for n in 0..split_vec.len() {
                    if split_vec[n].contains("\"}") {
                        let json_line = "{\"".to_owned() + &split_vec[n];
                        deserialize_jsonline(&json_line, &matches);
                    } else {
                        println!(
                            "{}Warning, non-JSON Log Found:{}\n{}\n",
                            color::Fg(color::Yellow),
                            color::Fg(color::Reset),
                            split_vec[n]
                        );
                    }
                }
            } else {
                println!(
                    "{}Warning, non-JSON Log Found:{}\n{}",
                    color::Fg(color::Yellow),
                    color::Fg(color::Reset),
                    &s
                );
            }
        }
    }
}
