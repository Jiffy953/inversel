use std::fs;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum BlockColor {
    Green, // Correct letter, correct position
    Yellow, // Correct letter, wrong position
    Gray, // Incorrect letter
}

fn load_words(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).unwrap();
    content.lines().map(|line| line.to_string()).collect()
}

struct WordFeedback {
    word: String,
    feedback: [BlockColor; 5],
}


fn matches_feedback(candidate: &str, feedback: &WordFeedback) -> bool {
    for (index, &color) in feedback.feedback.iter().enumerate() {
        match color {
            BlockColor::Green => {
                if candidate.chars().nth(index).unwrap() != feedback.word.chars().nth(index).unwrap() {
                    return false;
                }
            }
            BlockColor::Yellow => {
                if candidate.chars().nth(index).unwrap() == feedback.word.chars().nth(index).unwrap() || 
                   !feedback.word.contains(candidate.chars().nth(index).unwrap()) {
                    return false;
                }
            }
            BlockColor::Gray => {
                if feedback.word.contains(candidate.chars().nth(index).unwrap()) {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let words = load_words("resources/WordleWordDict.txt");
    println!("Loaded {} words.", words.len());

    let mut winning_word = String::new();
    println!("Enter the winning word:");
    std::io::stdin().read_line(&mut winning_word).expect("Failed to read line");
    let winning_word = winning_word.trim().to_string();

    let mut feedback_str = String::new();
    println!("Enter the feedback grid using characters (e.g., GYGRY) or emojis (e.g., ⬜⬜⬜🟨⬜):");
    std::io::stdin().read_line(&mut feedback_str).expect("Failed to read line");

    if feedback_str.trim().chars().count() % 5 != 0 || feedback_str.trim().chars().count() > 30 {
        panic!("Invalid feedback length. It must be multiples of 5 and up to 30 characters long.");
    }

    let feedback_arr: Vec<BlockColor> = feedback_str.trim().chars().map(|char| {
        match char {
            'G' | '🟩' => BlockColor::Green,
            'Y' | '🟨' => BlockColor::Yellow,
            'R' | '⬜' => BlockColor::Gray,
            _ => panic!("Invalid feedback character!"),
        }
    }).collect();

    let mut all_potential_guesses = Vec::new();    
    for chunk in feedback_arr.chunks(5) {
        let potential_guesses: Vec<_> = words.iter().filter(|&word| {
            let word_feedback = WordFeedback {
                word: winning_word.clone(),
                feedback: [
                    chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]
                ],
            };
            matches_feedback(&word, &word_feedback)
        }).collect();

        all_potential_guesses.push(potential_guesses);
    }

    for (index, guesses) in all_potential_guesses.iter().enumerate() {
        println!("Feedback {}: {:?}", index + 1, guesses);
    }
}
//clear
//RYRRRRYRRYRYYGYGYRRRGYYRRGGGGG
//RRGYRRGGGRRGGGRRGGGRGGGGG
