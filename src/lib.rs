extern "C" {
  fn c_timestring(hour: std::os::raw::c_int, buffer: *mut std::os::raw::c_char);
}

pub fn timestring(hour: bool) -> String {
    let mut buffer: Vec<u8> = vec![0;25];
    let c_hour: std::os::raw::c_int = if hour { 1 } else { 0 };
    unsafe { c_timestring(c_hour, buffer.as_mut_ptr() as *mut i8); }

    buffer.pop(); // remove nul byte
    String::from_utf8(buffer).unwrap()
}  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesn_t_segfault() {
        timestring(true);
        timestring(false);
    }
}

