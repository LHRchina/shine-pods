use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, HeaderBar, Button, ListBox, Label, ScrolledWindow, PolicyType};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::cell::RefCell;



const APP_ID: &str = "com.shinepods.player";



fn main() -> glib::ExitCode {
    
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let header_bar = HeaderBar::new();
    header_bar.set_title_widget(Some(&gtk::Label::new(Some("ShinePods music player"))));
    //let current_sink = Arc::new(RefCell::new(RefCell::new(None)));

    let list_box = ListBox::new();
    let music_files = std::fs::read_dir("store").unwrap();
    // Create a shared, mutable reference to the current sink
    //let current_sink = Arc::new(RefCell::new(RefCell::new(None)));



    // Create a vector to store the file paths
    let file_paths = Arc::new(RefCell::new(Vec::new()));
    
    //let current_sink_clone = Arc::clone(&current_sink);

    for file in music_files {
        let path = file.unwrap().path();
        let label = Label::new(Some(&path.file_name().unwrap().to_string_lossy()));
        list_box.append(&label);
        file_paths.borrow_mut().push(path);
    }

    // Clone file_paths for the closure
    let file_paths_clone = Arc::clone(&file_paths);

    list_box.connect_row_activated(move |_, row| {
        let index = row.index();
        let path = file_paths_clone.borrow()[index as usize].clone();
        let path_str = path.to_str().unwrap().to_owned();
        
        // Spawn a new thread to play the MP3
        std::thread::spawn(move || {
            if let Err(e) = play_mp3(&path_str) {
                eprintln!("Error playing file: {}", e);
            }
        });
    });

    let scrolled_window = ScrolledWindow::builder()
    .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
    .min_content_width(100)
    .min_content_height(200)
    .child(&list_box)
    .build();


    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fairy Player")
        .titlebar(&header_bar)
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();


            // Create a window



    // Present window
    window.present();
}


fn play_mp3(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    // Create a sink
    let sink = Sink::try_new(&stream_handle)?;

    // Open the file
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?;
    // Add the source to the sink
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

    Ok(())
}







