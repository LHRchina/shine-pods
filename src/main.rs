use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, HeaderBar, ListBox, Label, ScrolledWindow, PolicyType};
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

// Define the AudioPlayback struct
struct AudioPlayback {
    _stream: OutputStream, // Must keep this alive
    sink: Sink,
}

fn build_ui(app: &Application) {
    let header_bar = HeaderBar::new();
    header_bar.set_title_widget(Some(&gtk::Label::new(Some("ShinePods Music Player"))));

    let list_box = ListBox::new();
    let music_files = std::fs::read_dir("store").unwrap();

    // Create a shared, mutable reference to the current playback
    let current_playback: Arc<RefCell<Option<AudioPlayback>>> = Arc::new(RefCell::new(None));

    // Create a vector to store the file paths
    let file_paths = Arc::new(RefCell::new(Vec::new()));

    for file in music_files {
        let path = file.unwrap().path();
        let label = Label::new(Some(&path.file_name().unwrap().to_string_lossy()));
        list_box.append(&label);
        file_paths.borrow_mut().push(path);
    }

    // Clone shared resources for the closure
    let file_paths_clone = Arc::clone(&file_paths);
    let current_playback_clone = Arc::clone(&current_playback);

    list_box.connect_row_activated(move |_, row| {
        let index = row.index();
        let path = file_paths_clone.borrow()[index as usize].clone();
        let path_str = path.to_str().unwrap().to_owned();

        // Stop the current sink if it exists
        if let Some(playback) = &*current_playback_clone.borrow() {
            playback.sink.stop();
        }

        // Play the new MP3 and get the new playback object
        match play_mp3(&path_str) {
            Ok(playback) => {
                *current_playback_clone.borrow_mut() = Some(playback);
            }
            Err(e) => {
                eprintln!("Error playing file: {}", e);
            }
        }
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
        .title("ShinePods Music Player")
        .titlebar(&header_bar)
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}

fn play_mp3(path: &str) -> Result<AudioPlayback, Box<dyn std::error::Error>> {
    // Get an output stream handle to the default physical sound device
    let (stream, stream_handle) = OutputStream::try_default()?;
    // Create a sink
    let sink = Sink::try_new(&stream_handle)?;

    // Open the file
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?;
    // Add the source to the sink
    sink.append(source);

    // Do not block the thread
    // sink.sleep_until_end(); // Remove this line

    // Return both stream and sink to keep them alive
    Ok(AudioPlayback {
        _stream: stream,
        sink,
    })
}