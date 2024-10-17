use crate::editor::{
    gap_buffer::GapBuffer,
    key_handling::read_key,
};
use std::io::{self, Write};

pub mod file_handling;
pub mod key_handling;
pub mod general;
pub mod gap_buffer;


pub struct Editor {
    gap_buffer: GapBuffer,
    cursor_size: usize,
}

impl Editor {
    pub fn new(gap_buffer: GapBuffer) -> Self {
        Self {
            gap_buffer,
            cursor_size: 0,
        }
    }

    pub fn cycle(&mut self){
        loop {
            match read_key().unwrap(){
                'q' => {break;}
                '\x1b' => {
                    print!("AHAHAHAHA");
                }
                '\r' => {
                    self.cursor_size += 1;
                    self.gap_buffer.insert('\n');
                    self.handle_key_addition();
                },
                '\u{7f}' => {
                    self.gap_buffer.delete();
                    if self.cursor_size > 0 {
                        self.cursor_size -= 1;
                    }
                    self.handle_key_addition();
                }
                c => {
                    self.cursor_size += 1;
                    self.gap_buffer.insert(c);
                    self.handle_key_addition();
                }
            }
        }
    }

    fn handle_key_addition(&mut self){
        general::clear_screen();
        let buffer_text = self.gap_buffer.get_content();
        self.render_buffer(&buffer_text);
        let (x_cursor_pos,y_cursor_pos) = self.get_pos_by_cursor(&buffer_text);
        print!("\x1B[{y_cursor_pos};{x_cursor_pos}H");
        let _ = io::stdout().flush();
    }

    fn render_buffer(&mut self, buffer_text: &str){
        let mut buffer_vec = buffer_text.split("\n");
        if let Some(first_line) = buffer_vec.next(){
            print!("{}", first_line);
            for buffer in buffer_vec {
                print!("\r\n");
                print!("{}", buffer);
            }
        }
    }

    fn get_pos_by_cursor(&self, buffer_text: &str) -> (usize, usize){
        let mut y_cursor_pos = 0;
        let mut x_cursor_pos = 0;
        let buffer_vec = buffer_text.split("\n");
        let mut reference_cursor_x = self.cursor_size;

        let mut old_line_length = 0; 

        for line in buffer_vec {
            let current_line_length = line.len() + 1;
            //println!("line length {}", current_line_length);
            //println!("old line length {}", old_line_length);
            if ((reference_cursor_x) as isize - current_line_length as isize) <= 0 {
                println!("first1");
                x_cursor_pos = reference_cursor_x;
            }else{
                println!("second2");
                reference_cursor_x -= old_line_length;
                x_cursor_pos = reference_cursor_x;
            }
            //println!("reference cursor{}", reference_cursor_x);
            y_cursor_pos += 1;
            old_line_length = current_line_length;
        }
        println!("Cursor Size: {}", self.cursor_size);
        //println!("Total lines length:{}", total_lines_length);
        //println!("Start gap:{}", self.gap_buffer.gap_start);
        //println!("End gap:{}", self.gap_buffer.gap_end);
        print!("{:?}", self.gap_buffer.buffer);
        (x_cursor_pos,y_cursor_pos)
    }


}
