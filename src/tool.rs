
pub fn vk_to_string(i8_buf: &[i8]) -> String {
    let mut s = String::new();
    for &i in i8_buf {

        if i == 0 {
            break;
        }

        let c: char = (i as u8) as char;
        s.push(c);
    }

    s
}
