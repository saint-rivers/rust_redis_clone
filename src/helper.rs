use bytes::{BytesMut, Buf};

pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![]; // create a vector to store the final result

    let length = buf.len(); // get the total length of our input
    let mut word = "".to_string(); // this is the temp variable to store each parsed word

    for i in 0..length {
        // loop over each byte
        match buf.get_u8() {
            b' ' => {
                // if it reaches a whitespace, the word as finished and we can add the word to the vector of strings
                vec.push(word);
                // reset the temp word variable
                word = "".to_string();
            }
            other => {
                // if the next byte is other than a whitespace, add the byte to the word
                word.push(other as char);
                // if it's the last word, just push that instead
                let new = word.clone();
                if i == length - 1 {
                    vec.push(new);
                }
            }
        }
    }
    vec
}
