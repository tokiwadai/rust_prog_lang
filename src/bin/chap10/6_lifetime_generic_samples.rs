// It’s possible for structs to hold references,
// but in that case we would need to add a lifetime annotation
// on every reference in the struct’s de nitio, pp. 222
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// The lifetime parameter declaration after impl and its use after the type name are required,
// but we’re not required to annotate the lifetime of the reference to self
// because of the  rst elision rule, pp. 226
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// There are two input lifetimes, so Rust applies the 1st lifetime elision rule
// and gives both &self and announcement their own lifetimes.
// Then, because one of the parameters is &self, the return type gets the lifetime of &self,
// and all lifetimes have been accounted for, pp. 226
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    println!("i.level: {}", i.level());
    println!("announce_and_return_part: {}",
             i.announce_and_return_part("hello"));
}