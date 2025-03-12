use anyhow::Result;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};

fn read_corpus(corpus_path: &str) -> Result<String> {
    let corpus: String = fs::read_to_string(corpus_path)?;
    Ok(corpus)
}

#[allow(dead_code)]
fn split_with_separators_old(s: String, separator: String) -> Vec<String> {
    let split_vec: Vec<String> = s
        .split_inclusive(&separator)
        .map(|e| e.to_string())
        .collect();
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
    fully_split
}
fn split_with_separators(s: &str, separator: String) -> Vec<String> {
    let split_idx: Vec<_> = s.match_indices(&separator).collect();
    let mut walker = 0;
    let mut split_value: Vec<String> = Vec::new();
    for (idx, _v) in split_idx {
        if walker != idx {
            split_value.push(s[walker..idx].to_string());
        }
        split_value.push(separator.clone());
        walker = idx + separator.len();
    }
    if walker != s.len() {
        split_value.push(s[walker..].to_string());
    }

    split_value
}

#[allow(unused_assignments)]
fn tokenize(word: &str, vocabulary: &HashMap<String, usize>) -> Vec<String> {
    let mut sorted_vocab: Vec<(&str, usize)> =
        vocabulary.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    sorted_vocab.sort_by(|(k1, v1), (k2, v2)| v1.cmp(v2).then(k2.cmp(k1)));

    let mut tokenized_word_vector: Vec<(String, bool)> = vec![(word.to_string(), false)];
    let mut copy_of_working: Vec<(String, bool)> = tokenized_word_vector.clone();

    let mut temp_vector: Vec<(String, bool)> = Vec::new();
    for (token, _) in sorted_vocab.into_iter().rev() {
        let mut offset = 0;
        for (i, (w, t)) in tokenized_word_vector.iter().enumerate() {
            if !t {
                let tokenized_vector = split_with_separators(w, token.to_string());
                temp_vector = tokenized_vector
                    .into_iter()
                    .map(|e| if e == token { (e, true) } else { (e, false) })
                    .collect();
                let idx = i + offset;
                copy_of_working.remove(idx);
                copy_of_working.splice(idx..idx, temp_vector.clone());
                offset += temp_vector.len() - 1;
            }
        }
        tokenized_word_vector = copy_of_working.clone();
    }

    tokenized_word_vector.into_iter().map(|(v, _)| v).collect()
}

fn build_bpe_vocabulary(
    corpus: &str,
    vocabulary: &mut HashMap<String, usize>,
    token_frequency: usize,
) -> usize {
    let tokenized_output = tokenize(corpus, vocabulary);
    let mut potential_token: BTreeMap<String, usize> = BTreeMap::new();
    tokenized_output.windows(2).for_each(|c| {
        let new_token = c[0].to_string() + c[1].as_str();
        potential_token
            .entry(new_token)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    });
    let (mut token, mut freq) = ("".to_string(), 0);

    for (k, v) in potential_token.into_iter() {
        if v > freq {
            token = k;
            freq = v;
        }
    }
    if freq >= token_frequency {
        vocabulary.insert(token.clone(), token.len());
    }
    freq
}

fn main() -> Result<()> {
    let corpus_path = "data/corpus.txt";
    // let corpus_path = "data/test.txt";
    // let corpus_path = "data/input.txt";
    let get_user_input = false;

    let corpus = read_corpus(corpus_path)?;
    let mut vocabulary: HashMap<String, usize> = HashMap::new();
    corpus.chars().map(|c| c.to_string()).for_each(|s| {
        vocabulary.insert(s.clone(), s.len());
    });

    let mut word_to_tokenize = String::new();
    if get_user_input {
        io::stdin().read_line(&mut word_to_tokenize)?;
    } else {
        word_to_tokenize.push_str("testAAGAB bac");
    }
    println!("Corpus Size: {}", corpus.len());
    word_to_tokenize = corpus[..].to_string();

    // let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);
    // let string_output = format!("|{}", tokenized_output.join("|"));
    // println!("Output Vector:\n{:?}", tokenized_output);
    // println!("Tokenized Result:\n{}", string_output);

    loop {
        let len = build_bpe_vocabulary(&word_to_tokenize, &mut vocabulary, 2);
        if len < 10 {
            break;
        }
        if vocabulary.len() % 10 == 0 {
            println!("Vocab Length: {}", vocabulary.len());
        }
    }
    for (k, v) in vocabulary.iter() {
        println!("{}: {}", k, v);
    }
    let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);
    // println!("Output Vector:\n{:?}", tokenized_output);
    let string_output = format!("|{}", tokenized_output.join("|"));
    println!("Tokenized Result:\n{}", string_output);

    let mut file = fs::File::create("data/vocab.txt").unwrap();
    for (k, v) in vocabulary.iter() {
        writeln!(file, "{}: {}", k, v)?;
    }
    Ok(())
}
