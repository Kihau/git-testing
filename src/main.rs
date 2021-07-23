use std::{collections::HashMap, fs::File, io::Write, time::Instant};

fn main() {
    let now = Instant::now();
    word_occurrences();
    println!("Executed in: {:?}", now.elapsed());
}

fn word_occurrences() {
    let mut args = std::env::args();
    args.next();

    let path = match args.next() {
        Some(path) => path,
        None => panic!("Path to a file was not provided"),
    };

    let text = match std::fs::read_to_string(&path) {
        Ok(lines) => lines,
        Err(_) => panic!("Given path to file is incorrect"),
    };

    let mut map = HashMap::<String, usize>::new();
    let mut v: Vec<String> = Vec::new();

    const INCLUDED_CHARS: &[char] = &['`', '\''];
    let mut last_special = false;

    let mut buf = String::new();

    for c in text.chars() {
        let special = !c.is_alphabetic() && !INCLUDED_CHARS.contains(&c);

        if !last_special && special {
            if !buf.is_empty() && map.contains_key(&buf) {
                *map.get_mut(&buf).unwrap() += 1;
                v.push(buf.clone());
            } else if !buf.is_empty() {
                v.push(buf.clone());
                map.insert(buf, 1);
                buf = String::new();
            }
            buf.clear();
        } else if !special {
            buf.push(c.to_ascii_lowercase());
        }

        last_special = special;
    }

    if !buf.is_empty() && map.contains_key(&buf) {
        *map.get_mut(&buf).unwrap() += 1;
        v.push(buf.clone());
    } else if !buf.is_empty() {
        v.push(buf.clone());
        map.insert(buf, 1);
    }

    // Sort occurances and add them to  text file
    let mut hash_vec: Vec<(&String, &usize)> = map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut output = format!("Number of unique words: {:?}\n", hash_vec.len());
    output.push_str(format!("{} - unique word occurances:\n {:?}\n", path, hash_vec).as_str());

    // let mut output = format!("Number of unique words: {:?}\n", map.len());
    // output.push_str(format!("{} - unique word occurances:\n {:?}\n", path, map).as_str());

    let mut file = File::create("output_occurances.txt").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    // Sort all words and add them to a text file
    v.sort();
    let mut output = format!("Number of words: {:?}\n", v.len());
    output.push_str(format!("{} - Sorted words:\n {:?}\n", path, v).as_str());

    let mut file = File::create("output_sorted.txt").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
