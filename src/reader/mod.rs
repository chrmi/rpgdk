use std::fs;
use rand::Rng;

/// # Reads file and returns contents as a string
///
/// # Arguments
///
/// * ```file_name``` - Name of the file (with path)
///
/// # Example
///
/// ```
/// // Assumes names.txt is in the same folder as app.
/// let names = reader::read_file("names.txt");
/// ```
pub fn read_file(file_name: &str) -> String {
    let contents = fs::read_to_string(file_name)
        .expect("Error reading the file with file_read.");
    contents
}

/// # Picks a random substring between spaces
///
/// # Arguments
///
/// * ```file_contents``` - A string from a file (or not).
///
/// # Example
///
/// ```
/// // Assume names is string of values separated by spaces, i.e. "Delza Dezla Zedla".
/// let name = String::from(reader::random_word(&names));
/// ```
pub fn random_word(file_contents: &str) -> String {
    let mut words: Vec<String> = Vec::new();

    for word in file_contents.split(" ") {
        words.push(word.trim().to_string());
    }

    String::from(&words[rand::thread_rng().gen_range(0, words.len())])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_loads_test() {
        assert!(true, "Reader module loads");
    }

    #[test]
    fn read_file_test() -> Result<(), String> {
        let s = read_file("test.txt");
	if s.contains("cats and boots") {
            Ok(())
	} else {
            Err(String::from("file read failed"))
	}
    }

    #[test]
    fn random_word_test() {
        let content = read_file("test.txt");
        let random_word = random_word(&content);

        assert!(random_word == "cats" || 
                random_word == "and" || 
                random_word == "boots", 
                "Random word unexpected value: {}", random_word);
    }
}
