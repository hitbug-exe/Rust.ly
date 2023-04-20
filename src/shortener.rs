// import the Harsh library
use harsh::{Harsh, HarshBuilder};

// define a Shortener struct
pub struct Shortener {
    id: u64,
    generator: Harsh,
}

impl Default for Shortener {
    // implement the Default trait to create a new Shortener instance with default values
    fn default() -> Self {
        // initialize a new Harsh builder with a custom alphabet
        let builder = HarshBuilder::new().alphabet("hitbugexe".to_owned());
        Shortener {
            id: 0, // initialize the id to 0
            generator: builder.build().unwrap(), // build a Harsh instance with the builder
        }
    }
}

impl Shortener {
    // define a method to generate a new shortened URL
    fn next_id(&mut self) -> String {
        let hashed = self.generator.encode(&[self.id]); // encode the current id using Harsh
        self.id += 1; // increment the id for the next use
        hashed // return the encoded value as a String
    }
}

