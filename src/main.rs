use maxx_editor::editor::{
    Editor,
    general::{enter_raw_mode,leave_raw_mode},
    gap_buffer::GapBuffer,
};

fn main(){
    enter_raw_mode();
    let gap_buffer = GapBuffer::new(15);
    let mut editor = Editor::new(gap_buffer);
    editor.cycle();
    leave_raw_mode();
    //let mut gap_buffer = GapBuffer::new(3);
    //
    //println!("{:?}", gap_buffer.buffer);
    //gap_buffer.insert('a');
    //println!("{:?}", gap_buffer.buffer);
    //gap_buffer.insert('a');
    //println!("{:?}", gap_buffer.buffer);
    //gap_buffer.insert('a');
    //println!("{:?}", gap_buffer.buffer);
    //gap_buffer.insert('a');
    //println!("{:?}", gap_buffer.buffer);
    //println!("{:?}", gap_buffer.gap_start);
    //println!("{:?}", gap_buffer.gap_end);
    //gap_buffer.move_cursor(1);
    //println!("{:?}", gap_buffer.buffer);
    //println!("{:?}", gap_buffer.gap_start);
    //println!("{:?}", gap_buffer.gap_end);
    //gap_buffer.move_cursor(2);
    //println!("{:?}", gap_buffer.buffer);
    //println!("{:?}", gap_buffer.gap_start);
    //println!("{:?}", gap_buffer.gap_end);
    //
    //gap_buffer.move_cursor(3);
    //println!("{:?}", gap_buffer.buffer);
    //println!("{:?}", gap_buffer.gap_start);
    //println!("{:?}", gap_buffer.gap_end);
    //
    //gap_buffer.insert('b');
    //println!("{:?}", gap_buffer.buffer);
    //println!("{:?}", gap_buffer.gap_start);
    //println!("{:?}", gap_buffer.gap_end);
    //println!("{}",gap_buffer.get_content());
}
