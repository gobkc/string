pub trait StringExtend {
    fn substr(&self, start_pos: i32,end_pos:i32) -> String;
}

impl StringExtend for str {
    fn substr(&self, start_pos: i32,end_pos:i32) -> String {
        let slice = &self[start_pos..end_pos];
        slice.to_string()
    }
}
