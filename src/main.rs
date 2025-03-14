use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};

fn read_corpus(corpus_path: &str) -> Result<String> {
    let corpus: String = fs::read_to_string(corpus_path)?;
    Ok(corpus)
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

fn main() -> Result<()> {
    let corpus_path = "data/corpus.txt";
    // let corpus_path = "data/test.txt";
    // let corpus_path = "data/input.txt";
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
        if len < 10 {
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

    let mut file = fs::File::create("data/vocab.txt").unwrap();
    for (k, v) in vocabulary.iter() {
        writeln!(file, "{}: {}", k, v)?;
    }
    Ok(())
}
