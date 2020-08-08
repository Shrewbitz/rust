pub fn run(words: Vec<&str>, order: String) -> bool {
    // let mut result: bool = true;
    for i in 0..words.len()-1 {
        let word1 = &words[i];
        let word2 = &words[i+1];
        let short = if word1.len() > word2.len() {word2.len()} else {word1.len()};
        for letter in 0..short {
            let letter1 = word1.chars().nth(letter).unwrap();
            let letter2 = word2.chars().nth(letter).unwrap();
            let order1 = order.find(letter1);
            let order2 = order.find(letter2);
            if order1 < order2 {
                break;
            } else if order1 == order2 {
                if word2.len() == letter + 1 {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    // return result;
    return true;
}