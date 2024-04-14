pub mod widgets;
pub mod factory;

pub mod helper {
    pub fn get_substring(text: &str, pos: i32) -> String {
        if text.len() > pos as usize {
            let idx: usize = pos as usize;
    
            if let Some(n) = text.char_indices()
                .map(|x| x.0)
                .take(idx)
                .last() {
    
                let substring = text.split_at(n).0;
    
                return substring.to_string() + "...";
            }
        }
        
        text.to_string()
    }
}
