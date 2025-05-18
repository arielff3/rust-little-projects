use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
fn main() {
    let text = std::fs::read_to_string("src/text.txt").unwrap();
    let bindings = text.lines().map(|line| line.to_string()).collect::<Vec<_>>();
    let chunks = bindings.chunks(100);
    
    let word_counts = Arc::new(Mutex::new(HashMap::<String, usize>::new()));

    let mut handles: Vec<_> = vec![];

    for chunk in chunks {
        let chunk = chunk.to_vec();
        let word_counts = Arc::clone(&word_counts);


        let handle = thread::spawn(move || {
            let mut local_map = HashMap::new();

            for line in chunk {
                for word in line.split_whitespace() {
                    let word = word.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
                    *local_map.entry(word).or_insert(0) += 1;
                }
            }

            let mut global = word_counts.lock().unwrap();
            for (word, count) in local_map {
                *global.entry(word).or_insert(0) += count;
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_counts = word_counts.lock().unwrap();
    let mut sorted: Vec<_> = final_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top 10 palavras mais comuns:");
    for (word, count) in sorted.iter().take(10) {
        println!("{:<15} {}", word, count);
    }

}   
