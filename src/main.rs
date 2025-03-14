use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
#[allow(dead_code)]
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
#[allow(dead_code)]
fn character_level_tokenize(word: &str) -> Vec<String> {
    word.chars().map(|c| c.to_string()).collect()
}

fn merge_tokenized_corpus(tokenized_corpus: &Vec<String>, token: String) -> Vec<String> {
    let mut new_tokenized_corpus: Vec<String> = Vec::with_capacity(tokenized_corpus.len());
    let mut skip = false;
    tokenized_corpus.windows(2).for_each(|c| {
        let new_token = c[0].to_string() + c[1].as_str();
        if skip {
            skip = false;
        } else if new_token.cmp(&token) == Ordering::Equal {
            new_tokenized_corpus.push(new_token);
            skip = true;
        } else {
            new_tokenized_corpus.push(c[0].to_string());
        }
    });
    if !skip {
        new_tokenized_corpus.push(tokenized_corpus.last().unwrap().to_string());
    }
    new_tokenized_corpus
}
// fn merge_tokenized_corpus(tokenized_corpus: &[String], token: String) -> Vec<String> {
//     let mut new_tokenized_corpus = Vec::with_capacity(tokenized_corpus.len());
//     let mut i = 0;

//     while i < tokenized_corpus.len() {
//         if i < tokenized_corpus.len() - 1
//             && tokenized_corpus[i].clone() + &tokenized_corpus[i + 1] == token
//         {
//             new_tokenized_corpus.push(token.to_string());
//             i += 2; // Skip the next token since it was merged
//         } else {
//             new_tokenized_corpus.push(tokenized_corpus[i].clone());
//             i += 1;
//         }
//     }

//     new_tokenized_corpus
// }

fn build_bpe_vocabulary(
    tokenized_corpus: &Vec<String>,
    vocabulary: &mut HashMap<String, usize>,
    token_frequency: usize,
) -> (usize, Vec<String>) {
    let mut potential_token: BTreeMap<String, usize> = BTreeMap::new();
    tokenized_corpus.windows(2).for_each(|c| {
        if let Some(c) = c[1].clone().chars().last() {
            if c.is_whitespace() {
                return;
            }
        }
        if let Some(c) = c[1].clone().chars().next() {
            if c.is_whitespace() {
                return;
            }
        }
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
    (freq, merge_tokenized_corpus(tokenized_corpus, token))
}
// struct Data(HashMap<String, usize>);
fn main() -> Result<()> {
    let corpus_path = "data/corpus.txt";
    // let corpus_path = "data/input.txt";
    // let corpus_path = "data/test.txt";
    let vocab_path = "data/vocab.json";
    let v_path = Path::new(vocab_path);
    if v_path.exists() {
        let vocab_data = fs::read_to_string(v_path)?;
        let vocabulary: HashMap<String, usize> = serde_json::from_str(&vocab_data)?;
        println!("Enter a word to tokenize:");

        let mut word_to_tokenize = String::new();
        io::stdin().read_line(&mut word_to_tokenize)?;
        let tokenized_output = tokenize(&word_to_tokenize, &vocabulary);

        let string_output = format!("|{}", tokenized_output.join("|"));
        println!("Tokenized Result:\n{}", string_output);
        return Ok(());
    }

    let get_user_input = false;

    let corpus = read_corpus(corpus_path)?;
    let mut vocabulary: HashMap<String, usize> = HashMap::new();
    let mut tokenized_output: Vec<String> = Vec::new();
    corpus.chars().map(|c| c.to_string()).for_each(|s| {
        vocabulary.insert(s.clone(), s.len());
        tokenized_output.push(s.clone());
    });

    let mut word_to_tokenize = String::new();
    if get_user_input {
        io::stdin().read_line(&mut word_to_tokenize)?;
    } else {
        word_to_tokenize.push_str("testAAGAB bac");
    }
    println!("Corpus Size: {}", corpus.len());

    loop {
        let (len, new_tokenized_output) =
            build_bpe_vocabulary(&tokenized_output, &mut vocabulary, 2);
        if len < 20 {
            break;
        }
        if vocabulary.len() % 10 == 0 {
            println!("Vocab Length: {}", vocabulary.len());
        }
        tokenized_output = new_tokenized_output;
    }
    for (k, v) in vocabulary.iter() {
        println!("{}: {}", k, v);
    }

    let string_output = format!("|{}", tokenized_output.join("|"));
    println!("Tokenized Result:\n{}", string_output);

    let mut file = fs::File::create("data/vocab.json")?;
    let json_string = serde_json::to_string(&vocabulary)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
