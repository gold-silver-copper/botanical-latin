# botanical-latin "Decliner / conjugator / inflector for classical / botanical Latin."

[![Crates.io](https://img.shields.io/crates/v/botanical-latin.svg)](https://crates.io/crates/botanical-latin)
[![Documentation](https://docs.rs/botanical-latin/badge.svg)](https://docs.rs/botanical-latin/latest/botanical-latin/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/botanical-latin.svg)](https://crates.io/crates/botanical-latin) 


This crate provides a decliner/conjugator/inflector for classical / botanical Latin.

It uses data extracted from Wiktionary to create dictionaries from which it pulls words, if a word is not found in the dictionary it makes an educated guess as to the stem of the word (and gender for nouns), and declines accordingly. 
Right now Nouns and Adjectives are almost completely finished in their implementation, whereas Verbs only support the present tense and need a good bit more work, but it should be already useful for many tasks.

Sample usage: 

```rust

use botanical_latin::*;
fn main() {
  


    // Load the Latin conjugator with csv dictionaries, provided in the github repo
    let inflector = Latin::new(
        "nouns.csv".into(),
        "adjectives.csv".into(),
        "verbs.csv".into(),
    );


    // Optionally define a ComplexNoun to decline a Noun phrase that can include multiple adjectives and nouns in apposition
    let complexik = ComplexNoun {
        head_noun: "lorica".into(),
        adjective: vec!["hamatus".into(),"grandis".into()],
        adposition_noun: vec!["manica".into()],
    };

    // Inflect the complex noun as so. Returns a String
    let complex = inflector.complex_noun(&complexik, &Case::Abl, &Number::Singular);
    println!("{:#?}", complex);
    //Output: "lorica manica hamata grandi"


    // To conjugate a noun by itself use the noun() function from the inflector, it output a tuple that contains the inflected string as the first(0th) element, and the Gender as the second element
    let noun = inflector.noun("agricola", &Case::Acc, &Number::Plural);
    println!("{:#?}", noun.0);
    //Output: "agricolas"

    //Adjectives are similar to nouns, but require an additional Gender argument. Returns an inflected String
    let adj = inflector.adjective("integer", &Case::Nom, &Number::Singular,&Gender::Feminine);
    println!("{:#?}", adj);
    //Output: "integra"

    //You can guess nouns and adjectives without instantiating the conjugator with dictionaries if you so desire. But instantiating with the csv dictionaries gives a superior result.
    let guessed_adjective = Latin::guess_adjective("schoenoides", &Case::Gen, &Number::Plural, &Gender::Feminine);
    println!("{:#?}", guessed_adjective);
    // Output: "schoenoidum"

    let guessed_noun = Latin::guess_noun("hibiscus", &Case::Gen, &Number::Plural);
    println!("{:#?}", guessed_noun.0);
    //Output: "hibiscorum"



}




```