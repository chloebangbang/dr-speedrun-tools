mod ralsei;

use win_hotkeys::VKey;

const CONF_FILE: &str = "quiz-tracker.json";

fn main() {
    pretty_env_logger::init();

    // set up the ralsei tracker
    let mut tracker = ralsei::Tracker::load();

    // setup hotkey stuff
    let mut hkm = win_hotkeys::HotkeyManager::new(); 
    hkm.register_hotkey(VKey::Vk1, &[VKey::Shift, VKey::Control], || 1).unwrap();
    hkm.register_hotkey(VKey::Vk2, &[VKey::Shift, VKey::Control], || 2).unwrap();
    hkm.register_hotkey(VKey::Vk3, &[VKey::Shift, VKey::Control], || 3).unwrap();
    hkm.register_hotkey(VKey::Vk4, &[VKey::Shift, VKey::Control], || 4).unwrap();

    let (tx, rx) = crossbeam::channel::unbounded();
    hkm.register_channel(tx);

    std::thread::spawn(move || {
        hkm.event_loop();
    });

    println!("Entering event loop!");
    loop {
        tracker.save_obs_files();
        tracker.update(rx.recv().unwrap());
        dbg!(&tracker);
    }
}
