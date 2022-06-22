// design_pattern1.rs
// Execute the command `rustlings hint design_pattern1` for a hint.

// I AM NOT DONE

fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

// Don't change anything below this line.

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));

}