/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> isize {
    //unimplemented!("Score {} in Scrabble.", word);
    let score = 0;
    let mut scores = 0;
    for i in word.to_lowercase().chars(){
        //if i == 'ß'{scores -= 0}
        if i == 'a'{scores += 1}
        if i == 'e'{scores += 1}
        if i == 'i'{scores += 1}
        if i == 'o'{scores += 1}
        if i == 'u'{scores += 1}
        if i == 'l'{scores += 1}
        if i == 'n'{scores += 1}
        if i == 'r'{scores += 1}
        if i == 's'{scores += 1}
        if i == 't'{scores += 1}
        if i == 'd'{scores += 2}
        if i == 'g'{scores += 2}
        if i == 'b'{scores += 3}
        if i == 'c'{scores += 3}
        if i == 'm'{scores += 3}
        if i == 'p'{scores += 3}
        if i == 'f'{scores += 4}
        if i == 'h'{scores += 4}
        if i == 'v'{scores += 4}
        if i == 'w'{scores += 4}
        if i == 'y'{scores += 4}
        if i == 'k'{scores += 5}
        if i == 'j'{scores += 8}
        if i == 'x'{scores += 8}
        if i == 'q'{scores += 10}
        if i == 'z'{scores += 10}
        
        else{}

    }score + scores
}
