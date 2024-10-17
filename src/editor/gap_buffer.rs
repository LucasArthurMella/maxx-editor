pub struct GapBuffer {
    pub buffer: Vec<char>,
    pub gap_start: usize,
    pub gap_end: usize
}

impl GapBuffer {
    pub fn new(capacity: usize) -> Self{
        Self {
            buffer: vec!['\0'; capacity],
            gap_start: 0,
            gap_end: capacity - 1
        }
    }

    pub fn insert(&mut self, c: char){
        if self.gap_start == self.gap_end {
            self.resize();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn delete(&mut self){
        if self.gap_start != 0 {
            self.buffer.remove(self.gap_start-1);
            self.gap_start -= 1;
            self.gap_end -= 1;
        }
    }

    pub fn buffer_size(&self) -> usize{
        self.buffer.len() - (self.gap_end - self.gap_start) - 1
    }

    pub fn resize(&mut self){
        let resize_size = self.buffer_size() * 2;
        let mut gap_buffer = vec!['\0'; resize_size];
        
        let left_buffer = self.buffer[0..self.gap_start].to_owned();
        let mut right_buffer = self.buffer[self.gap_end..self.buffer.len()].to_owned();

        let mut new_buffer = left_buffer;
        self.gap_start = new_buffer.len();

        new_buffer.append(&mut gap_buffer);
        self.gap_end = new_buffer.len();

        new_buffer.append(&mut right_buffer);
        self.buffer = new_buffer;
    }

    pub fn move_cursor(&mut self, pos: usize){
        if pos < self.gap_start {
            let mut new_buffer = self.buffer[0..pos].to_owned();
            let new_gap_start = new_buffer.len();
            new_buffer.extend_from_slice(&self.buffer[self.gap_start..self.gap_end+1]);
            let new_gap_end = new_buffer.len() - 1;
            new_buffer.extend_from_slice(&self.buffer[pos..self.gap_start]);
            new_buffer.extend_from_slice(&self.buffer[self.gap_end+1..self.buffer.len()]);
            self.buffer = new_buffer;
            self.gap_start = new_gap_start;
            self.gap_end = new_gap_end;
        }else if pos > self.gap_start {
            let mut new_buffer = self.buffer[0..self.gap_start].to_owned();
            new_buffer.extend_from_slice(&self.buffer[self.gap_end+1..self.gap_end + (pos-self.gap_start+1)]);
            let new_gap_start = new_buffer.len();
            new_buffer.extend_from_slice(&self.buffer[self.gap_start..self.gap_end+1]);
            let new_gap_end = new_buffer.len() - 1;
            new_buffer.extend_from_slice(&self.buffer[self.gap_end + (pos-self.gap_start+1)..self.buffer.len()]);
            self.buffer = new_buffer;
            self.gap_start = new_gap_start;
            self.gap_end = new_gap_end;
        }
    }

    pub fn get_content(&self) -> String {
        let mut content = String::with_capacity(self.buffer.len());
        content.push_str(&self.buffer[0..self.gap_start].iter().collect::<String>());

        if let Some(_i) = self.buffer.get(self.gap_end+1) {
            content.push_str(&self.buffer[self.gap_end+1..self.buffer.len()].iter().collect::<String>());
        } 
        content
    }


}
