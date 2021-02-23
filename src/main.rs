use std::{fs::read_to_string, rc::Rc, sync::Arc, thread};
use rand::{self, Rng, thread_rng};
use warp::Filter;

#[tokio::main]
async fn main() {
    let data = Arc::new(read_to_string("shakespeare.txt").expect("Could not read shakespeare.txt file"));
    println!("✅ Successfully read shakespeare.txt");
    let mut all_lines = Vec::new();
    for raw_line in data.split("\n") {
        let trimmed = raw_line.trim();
        if trimmed.len() > 0 {
            all_lines.push(trimmed.to_string());
        }
    }
    drop(data);
    println!("✅ Successfully loaded trimmed lines into vector");
    let hamlet_start = all_lines.iter().position(|s| *s == "HAMLET, PRINCE OF DENMARK");
    let hamlet_end = all_lines.iter().position(|s| *s == "JULIUS CAESAR");
    if let (Some(start), Some(end)) = (hamlet_start, hamlet_end) {
        all_lines.drain(start..end);
        println!("✅ Successfully removed Hamlet from lines");
    }
    else {
        println!("❌ Could not remove Hamlet from lines")
    }
    println!("✅ Successfully completed data initialization");

    let all_lines = warp::any().map(move || all_lines.clone());
    let routes = warp::path!(usize)
        .and(all_lines)
        .map(|count: usize, lines: Vec<String> | pick_random_lines(&lines, count) );
    
    println!("✅ Successfully created routes...launching server");
    warp::serve(routes)
        .run(([0,0,0,0], 8080))
        .await;
}

fn pick_random_lines(all_lines: &[String], count: usize) -> String {
    println!("✉️ Request for {} lines received", count);
    let mut gen = rand::thread_rng();
    let index: usize = gen.gen_range(0..all_lines.len()-count);
    let mut out = String::new();
    for i in index..index+count {
        if let Some(s) = all_lines.get(i) {
            out.push_str(s);
            out.push_str("\n");
        }
    }
    out
}