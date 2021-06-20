use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::Command;

type AdjacencyMatrix = HashMap<String, HashMap<String, u32>>;
type FlatAdjacencyMatrix = HashMap<String, Vec<String>>;

/// Get the content of the file, remove newlines and collapses sequences of spaces, then collects the content into a vector.
///
/// # Examples
///
/// ```
/// use textgen::get_file_content;
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// ```
fn get_corpus(input: String, inline_mode: bool) -> Vec<char> {
    let mut corpus = String::new();

    if inline_mode {
        corpus = input;
    } else {
        let mut buf_reader = BufReader::new(File::open(input).unwrap());
        buf_reader.read_to_string(&mut corpus).unwrap();
    }

    corpus = corpus
        .replace("\n", " ")
        .replace("\u{ad}", " ")
        .trim()
        .to_string();

    loop {
        let new_corpus = corpus.replace("  ", " ");

        if corpus.len() == new_corpus.len() {
            break;
        } else {
            corpus = new_corpus;
        }
    }

    corpus.chars().collect::<Vec<char>>()
}

/// Returns a random value from an [type@FlatAdjacencyMatrix].
///
/// # Examples
///
/// ```
/// use textgen::{get_rand_value, get_file_content};
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// let flat_matrix = flatten_adjacency_matrix(&create_weighted_adjacency_matrix(&corpus, 3, 2));
/// let mut sentence: String = String::new();
/// let mut cmd = Command::new("sh");
/// cmd.args(&["-c", "od -vAn -N4 -tu4 < /dev/urandom"]);
///
/// let value = get_rand_value(cmd, flat_matrix, 3, &sentence);
/// ```
fn get_rand_value(
    cmd: &mut Command,
    flat_matrix: &FlatAdjacencyMatrix,
    key_len: u8,
    sentence: &str,
) -> String {
    let possible_values = flat_matrix
        .get(
            &sentence
                .chars()
                .skip(sentence.chars().count() - key_len as usize)
                .collect::<String>(),
        )
        .unwrap();

    possible_values[get_rand(cmd, possible_values.len())].clone()
}

/// Generates a sentence of values from an [type@FlatAdjacencyMatrix]. If possible, starts with a capitalized letter and ends with a dot.
///
/// # Examples
///
/// ```
/// use textgen::{get_rand_value, get_file_content, flatten_adjacency_matrix, create_weighted_adjacency_matrix};
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// let flat_matrix = flatten_adjacency_matrix(&create_weighted_adjacency_matrix(&corpus, 3, 2));
/// let mut cmd = Command::new("sh");
/// cmd.args(&["-c", "od -vAn -N4 -tu4 < /dev/urandom"]);
/// let capitalized_start: Vec<String> = flat_matrix
/// 	.keys()
/// 	.filter(|s| s.chars().next().unwrap().is_uppercase())
/// 	.cloned()
/// 	.collect();
///
/// let sentence = gen_sentence(&mut cmd, &flat_matrix, &capitalized_start, 3);
/// ```
fn gen_sentence(
    cmd: &mut Command,
    flat_matrix: &FlatAdjacencyMatrix,
    capitalized_start: &[String],
    key_len: u8,
) -> String {
    let mut sentence: String = String::new();

    if capitalized_start.is_empty() {
        sentence += &flat_matrix
            .keys()
            .nth(get_rand(cmd, flat_matrix.len()))
            .unwrap()
            .clone();
    } else {
        sentence += &capitalized_start[get_rand(cmd, capitalized_start.len())].clone();
    }

    for _i in 0..u64::MAX {
        let next_seq = get_rand_value(cmd, flat_matrix, key_len, &sentence);

        sentence += &next_seq.clone();

        if next_seq.ends_with('.') {
            break;
        }
    }

    sentence
}

// replace by : https://crates.io/crates/getrandom
#[doc(hidden)]
fn get_rand(cmd: &mut Command, max: usize) -> usize {
    let output = cmd.output().expect("failed to execute process").stdout;

    let mut output = String::from_utf8(output).unwrap();
    output.pop();

    output.trim_start().parse::<usize>().unwrap() % max
}

/// Creates an [type@AdjacencyMatrix] with sequence of chars as keys and a weighted list of sequence of chars as values.
///
/// # Examples
///
/// ```
/// use textgen::{get_file_content, create_weighted_adjacency_matrix};
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// let matrix = &create_weighted_adjacency_matrix(&corpus, 3, 2);
/// ```
fn create_weighted_adjacency_matrix(corpus: &[char], key_len: u8, val_len: u8) -> AdjacencyMatrix {
    let mut adjacency = AdjacencyMatrix::new();

    for chars in corpus.windows((key_len + val_len).into()) {
        let key: String = chars[0..key_len as usize].iter().collect();
        let val: String = chars[key_len as usize..(key_len + val_len).into()]
            .iter()
            .collect();

        if let Some(char_map) = adjacency.get_mut(&key) {
            *char_map.entry(val).or_insert(0) += 1;
        } else {
            adjacency.insert(key, [(val, 1)].iter().cloned().collect());
        }
    }

    adjacency
}

/// Flattens the matrix by collapsing the sequence of chars in the values by their weight.
///
/// # Examples
///
/// ```
/// use textgen::{get_file_content, flatten_adjacency_matrix, create_weighted_adjacency_matrix};
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// let flat_matrix = flatten_adjacency_matrix(&create_weighted_adjacency_matrix(&corpus, 3, 2));
/// ```
fn flatten_adjacency_matrix(matrix: &AdjacencyMatrix) -> FlatAdjacencyMatrix {
    let mut _matrix = FlatAdjacencyMatrix::new();

    for (k0, v0) in matrix.iter() {
        let mut serie = Vec::new();

        for (k1, v1) in v0.iter() {
            for _i in 0..*v1 {
                serie.push(k1.to_string());
            }
        }

        _matrix.insert(k0.to_string(), serie);
    }

    _matrix
}

/// Generates the output from the [type@FlatAdjacencyMatrix], depending on the mode.
///
/// # Examples
///
/// ```
/// use textgen::{get_file_content, flatten_adjacency_matrix, create_weighted_adjacency_matrix};
///
/// let corpus = get_file_content("myfile.txt".into(), true);
/// let flat_matrix = flatten_adjacency_matrix(&create_weighted_adjacency_matrix(&corpus, 3, 2));
///
/// let sequences = generate_sequences(&flat_matrix, true | !corpus.contains(&'.'), 5, 3)
/// ```
fn generate_sequences(
    flat_matrix: &FlatAdjacencyMatrix,
    token_mode: bool,
    count: u32,
    key_len: u8,
) -> String {
    let mut cmd = Command::new("sh");
    cmd.args(&["-c", "od -vAn -N4 -tu4 < /dev/urandom"]);

    if token_mode {
        let mut sequence = flat_matrix
            .keys()
            .nth(get_rand(&mut cmd, flat_matrix.len()))
            .unwrap()
            .clone();

        for _i in 0..count {
            sequence += &get_rand_value(&mut cmd, &flat_matrix, key_len, &sequence);
        }

        sequence
    } else {
        // https://lib.rs/crates/rayon
        let mut sentences = Vec::new();
        let capitalized_start: Vec<String> = flat_matrix
            .keys()
            .filter(|s| s.chars().next().unwrap().is_uppercase())
            .cloned()
            .collect();

        for _i in 0..count {
            sentences.push(gen_sentence(
                &mut cmd,
                &flat_matrix,
                &capitalized_start,
                key_len,
            ));
        }

        sentences.join(" ")
    }
}

/// Extracts the corpus, creates the matrix and produce the output.
///
/// # Examples
///
/// ```
/// use textgen::generate;
///
/// let output = generate("myfile.txt".into(), false, 3, 2, 5, false, false);
/// ```
pub fn generate(
    input: String,
    inline_mode: bool,
    key_len: u8,
    val_len: u8,
    count: u32,
    token_mode: bool,
    source: bool,
) -> Result<(), String> {
    let corpus = get_corpus(input, inline_mode);

    if corpus.is_empty() {
        Err("The input corpus must not be empty.".to_string())
    } else if corpus.len() < (key_len + val_len).into() {
        Err(format!(
            "The input corpus must have more than {} characters.",
            key_len + val_len
        ))
    } else {
        let matrix = create_weighted_adjacency_matrix(&corpus, key_len, val_len);

        if source {
            println!("{:#?}", matrix);
        } else {
            let flat_matrix = flatten_adjacency_matrix(&matrix);

            println!(
                "{}",
                generate_sequences(
                    &flat_matrix,
                    token_mode | !corpus.contains(&'.'),
                    count,
                    key_len
                )
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
