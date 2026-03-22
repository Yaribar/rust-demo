/*
Hugging Face Rust library to analyzes lyrics to songs and puts them into a sqlite database.
*/

// Struct holding a classification result: label text and confidence score
use rust_bert::pipelines::sequence_classification::Label;
// Model that classifies text into arbitrary categories without prior training on those categories
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
// Opens files from the filesystem (e.g., reading lyric files from disk)
use std::fs::File;
// Trait that adds the lines() method for reading input line by line
use std::io::BufRead;
// Wraps a reader with a buffer for efficient line-by-line file reading
use std::io::BufReader;

pub fn create_db() -> sqlite::Connection {
    let db = sqlite::open(":memory:").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('pop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('hip hop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('country')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('latin')")
        .unwrap();
    db
}

// Retrieves all genre labels from the zeroshotcandidates table and returns them as a Vec<String>
pub fn get_all_zeroshotcandidates() -> Vec<String> {
    // Create a fresh in-memory database pre-populated with genre labels
    let db = create_db();
    // SQL query to select every label from the table
    let query = "SELECT label FROM zeroshotcandidates";
    // Mutable vector to collect the labels as we iterate over rows
    let mut candidates: Vec<String> = Vec::new();
    // Iterate over each row returned by the query
    db.iterate(query, |pairs| {
        // Each row is a slice of (column_name, value) pairs
        for &(_column, value) in pairs.iter() {
            // Unwrap the value (it's an Option<&str>) and push it into the vector
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        // Return true to continue iterating to the next row
        true
    })
    .unwrap();
    // Return the collected candidate labels
    candidates
}

// Read lyrics from a file and return a vector of strings
pub fn read_lyrics(file: &str) -> Vec<String> {
    // Mutable vector to collect each line of lyrics
    let mut lyrics: Vec<String> = Vec::new();
    // Open the file, panic with a message if it doesn't exist
    let file = File::open(file).expect("Unable to open file");
    // Wrap the file in a BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);
    // Iterate over each line in the file
    for line in reader.lines() {
        // Unwrap the line and push it into the lyrics vector
        let line = line.unwrap();
        lyrics.push(line);
    }
    // Return the collected lyrics
    lyrics
}

/*
Use Hugging Face to classify lyrics using zero shot classification
and store them in an in-memory SQLite database.
Accepts a vector of strings as lyrics and grabs candidates from the in-memory SQLite database.
*/
pub fn classify_lyrics(lyrics: Vec<String>) -> Vec<Vec<Label>> {
    // Extract candidate labels from the SQLite database and put them in an array
    let temp_candidates = get_all_zeroshotcandidates();
    let candidate_labels: Vec<&str> = temp_candidates.iter().map(|s| s.as_str()).collect();
    // Join lyrics into a single string
    let lyrics: String = lyrics.join(" ");
    // Convert to type &str so it can be passed to the model
    let lyrics: &str = lyrics.as_ref();
    // Create the zero shot classification model with default config
    let zero_shot_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    // Run the model: classify the lyrics against the candidate labels
    let output = zero_shot_model.predict_multilabel([lyrics], candidate_labels, None, 128);
    // Return the classification results
    output
}