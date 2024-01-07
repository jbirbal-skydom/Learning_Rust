
use std::env;

fn main() {
    env::set_var("SLINT_DEBUG_PERFORMANCE", "refresh_full_speed,overlay");
    env::set_var("SLINT_BACKEND", "winit-software");


    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {


component MemoryTile inherits Rectangle {
    width: 64px;
    height: 64px;
    background: #3960D5;

    Image {
        source: @image-url("icons/bus.png");
        width: parent.width;
        height: parent.height;
    }
}

export component MainWindow inherits Window {
    MemoryTile {}
}

}