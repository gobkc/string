pub trait StringExtend {
    fn substr(&self, start_pos: usize, end_pos:usize) -> String;
}

impl StringExtend for str {
    fn substr(&self, start_pos: usize, end_pos:usize) -> String {
        let slice = &self[start_pos..end_pos];
        slice.to_string()
    }
}
