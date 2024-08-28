use emergency_backup::util;

mod backup;

fn main() {
    initialize();
}

fn initialize() {
    backup::open_window();
}
