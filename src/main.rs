use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io;

fn read_corpus(corpus_path: &str) -> Result<String> {
    let corpus: String = fs::read_to_string(corpus_path)?;
    Ok(corpus)
}

fn split_with_separators(s: String, separator: String) -> Vec<String> {
    let split_vec: Vec<String> = s
        .split_inclusive(&separator)
        .map(|e| e.to_string())
        .collect();
    // println!("{:?}", &split_vec);
    let mut fully_split: Vec<String> = Vec::new();
    for word in split_vec {
        if word.contains(&separator) {
            let sep_idx = word.len() - separator.len();
            if sep_idx != 0 {
                fully_split.push(word[..sep_idx].to_string());
            }
            fully_split.push(word[sep_idx..].to_string());
        } else {
            fully_split.push(word);
        }
    }
    // println!("{:?}", &fully_split);
    fully_split
}

fn tokenize(word: &str, vocabulary: &HashMap<String, usize>) -> Vec<String> {
    // println!("Word to tokenize: {}", word);
    // println!("-------\nVocabulary\n{:?}\n------", vocabulary.keys());

    let mut sorted_vocab: Vec<(&str, usize)> =
        vocabulary.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    sorted_vocab.sort_by_key(|(_, v)| *v);

    let mut tokenized_word_vector: Vec<(String, bool)> = vec![(word.to_string(), false)];
    let mut copy_of_working: Vec<(String, bool)> = tokenized_word_vector.clone();
    let mut temp_vector: Vec<(String, bool)> = Vec::new();
    for (token, length) in sorted_vocab.into_iter().rev() {
        // println!("{token}");
        let mut offset = 0;
        for (i, (w, t)) in tokenized_word_vector.iter().enumerate() {
            if !t {
                let tokenized_vector = split_with_separators(w.to_string(), token.to_string());
                // println!("-----\niter {i}\n{:?}", tokenized_vector);
                temp_vector = tokenized_vector
                    .into_iter()
                    .map(|e| if e == token { (e, true) } else { (e, false) })
                    .collect();
                // println!("-----\niter {i} Token {token}\n{:?}", temp_vector);
                let idx = i + offset;
                copy_of_working.remove(idx);
                // println!("{:?}", copy_of_working);
                copy_of_working.splice(idx..idx, temp_vector.clone());
                offset += temp_vector.len() - 1;
                // println!("After: {:?}", copy_of_working);
            }
        }
        tokenized_word_vector = copy_of_working.clone();
        // println!(
        //     "For Token {token}\n, length of tokenized == {}",
        //     tokenized_word_vector.len()
        // );
        // for (w, t) in tokenized_word_vector.iter() {
        //     println!("Token :{}   ", w);
        // }
    }
    // for (w, t) in tokenized_word_vector.iter() {
    //     println!("Token :{}   ", w);
    // }
    // println!("{:?}", tokenized_word_vector);
    tokenized_word_vector.into_iter().map(|(v, _)| v).collect()
}

fn main() -> Result<()> {
    // let corpus_path = "data/corpus.txt";
    let corpus_path = "data/test.txt";
    let get_user_input = false;

    let corpus = read_corpus(corpus_path)?;
    let mut vocabulary: HashMap<String, usize> = HashMap::new();
    corpus
        .chars()
        // .into_iter()
        .map(|c| c.to_string())
        .for_each(|s| {
            vocabulary.insert(s.clone(), s.len());
        });

    let mut word_to_tokenize = String::new();
    if get_user_input {
        io::stdin().read_line(&mut word_to_tokenize)?;
    } else {
        // word_to_tokenize.push_str("Singularity");
        word_to_tokenize.push_str("testAAGAB bac");
    }
    // word_to_tokenize = corpus.trim().to_string().clone();
    word_to_tokenize = corpus.to_string().clone();
    vocabulary.insert("AAGAB".to_string(), 5);
    // word_to_tokenize.split_at

    let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);
    let string_output = tokenized_output.join("|");
    println!("{:?}", tokenized_output);
    println!("{}", string_output);

    // split_with_separators(word_to_tokenize, "AAGAB".to_string());

    Ok(())
}
