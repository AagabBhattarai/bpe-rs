use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

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
    sorted_vocab.sort_by(|(k1, v1), (k2, v2)| v1.cmp(v2).then(k2.cmp(k1)));
    // sorted_vocab.sort_by_key(|(_, v)| *v);
    println!("SORTED VOCAB {:?}", sorted_vocab);

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

fn build_bpe_vocabulary(
    corpus: &str,
    vocabulary: &mut HashMap<String, usize>,
    // vocab_size: usize,
    token_frequency: usize,
) {
    let tokenized_output = tokenize(corpus, vocabulary);
    let mut potential_token: HashMap<String, usize> = HashMap::new();
    tokenized_output.windows(2).for_each(|c| {
        let new_token = c[0].to_string() + c[1].as_str();
        potential_token
            .entry(new_token)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    });
    let mut potential_token_vec: Vec<(String, usize)> = potential_token.into_iter().collect();
    // potential_token_vec.sort_by_key(|(_, v)| *v);
    potential_token_vec.sort_by(|(k1, v1), (k2, v2)| v2.cmp(v1).then(k1.cmp(k2)));
    // potential_token_vec.sort_by_key(|(w, _)| w.clone());

    println!("MAX FREQ VOCABA{:?}", potential_token_vec);
    // let (token, len) = potential_token_vec.pop().unwrap();
    // let mut max_freq_tokens: Vec<(String, usize)> = potential_token_vec
    //     .into_iter()
    //     .filter(|(_, v)| *v == len)
    //     .collect();
    // max_freq_tokens.push((token.clone(), len));
    // max_freq_tokens.sort_by_key(|(w, _)| w.clone());
    // println!("MAX FREQ: {:?}", max_freq_tokens);
    // let (token, _) = max_freq_tokens.remove(0);
    let (token, len) = potential_token_vec.remove(0);
    if len >= token_frequency {
        vocabulary.insert(token.clone(), token.len());
    }
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
    // vocabulary.insert("AAGAB".to_string(), 5);
    // word_to_tokenize.split_at

    let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);
    let string_output = format!("|{}", tokenized_output.join("|"));
    println!("Output Vector:\n{:?}", tokenized_output);
    println!("Tokenized Result:\n{}", string_output);

    // while vocabulary.len() <= 10 {
    build_bpe_vocabulary(&corpus, &mut vocabulary, 2);
    build_bpe_vocabulary(&corpus, &mut vocabulary, 2);
    println!("Vocab Length{}", vocabulary.len());
    // }
    for (k, v) in vocabulary.iter() {
        println!("{}: {}", k, v);
    }
    let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);
    println!("Output Vector:\n{:?}", tokenized_output);

    let mut file = fs::File::create("data/vocab.txt").unwrap();
    for (k, v) in vocabulary.iter() {
        writeln!(file, "{}: {}", k, v)?;
    }

    // split_with_separators(word_to_tokenize, "AAGAB".to_string());

    Ok(())
}
