use egui_file_dialog::{DialogState, FileDialog};
use egui_sf2g::{
    SfEgui, egui,
    sf2g::{
        graphics::RenderWindow,
        window::{Event, Style},
    },
};

fn main() {
    let mut win =
        RenderWindow::new([640, 480], "efpick", Style::NONE, &Default::default()).unwrap();
    win.set_framerate_limit(60);
    let mut sf_egui = SfEgui::new(&win);
    let mut file_dia = FileDialog::new().default_size(egui::vec2(640.0, 408.0));
    file_dia.pick_multiple();

    while win.is_open() {
        while let Some(ev) = win.poll_event() {
            sf_egui.add_event(&ev);
            if let Event::Closed = ev {
                win.close();
            }
        }
        let di = sf_egui
            .run(&mut win, |rw, ctx| {
                file_dia.update(ctx);
                if let Some(paths) = file_dia.take_picked_multiple()
                    && let Some(path) = paths.first()
                {
                    print!("{}", path.display());
                }
                if *file_dia.state() != DialogState::Open {
                    rw.close();
                }
            })
            .unwrap();
        sf_egui.draw(di, &mut win, None);
        win.display();
    }
}
