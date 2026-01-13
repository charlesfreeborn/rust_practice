fn main() {
    let s: String = String::from("hello, hackquest.");

    // starting from 0 represents 1 or more indexes.
    let slice1: &str = &s[0..2];
    // default starts from 0
    let slice2: &str = &s[..2];

    let len: usize = s.len();
    // contains the last byte. Since the index of the last byte us (len-1), the [4..len] method just contains the (len-1)th byte.
    let slice3: &str = &s[4..len];
    // default to the last byte
    let slice4: &str = &s[4..];

    // Get a slice of the entire string
    let slice5: &str = &s[0..len];
    // same as above
    let slice6: &str = &s[..];
}
