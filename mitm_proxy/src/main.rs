mod mitm_proxy;
mod requests;

use std::{
    sync::mpsc::{channel, sync_channel},
    thread,
};

use crate::mitm_proxy::MitmProxy;

use eframe::{
    egui::{self, CentralPanel, Vec2},
    run_native, App,
};
use proxyapi::ProxyAPI;
use tokio::runtime::Runtime;

static X: f32 = 980.;
static Y: f32 = 960.0;
static PADDING: f32 = 20.;

// fn fetch_requests(){
//     ProxyAPI::new().fetch();
// }

impl App for MitmProxy {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.manage_theme(ctx);

        self.render_top_panel(ctx, frame);

        if self.check_listener() {
            CentralPanel::default().show(ctx, |ui| self.render_columns(ui));
            self.fetch_requests();
        } else {
            CentralPanel::default().show(ctx, |ui| ui.label("waiting for connection"));
        }
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(X, Y));

    // create the app with listener false
    // update listener when it is true

    let (listener_tx, listener_rx) = sync_channel(1);
    let mut rt = Runtime::new().unwrap();

    thread::spawn(move || {
        rt.block_on( async move {
            loop {
                if let Ok(proxy_api) = ProxyAPI::new().await {
                    listener_tx.send(proxy_api);
                }
            }
        })
    });

    run_native(
        "Man In The Middle Proxy",
        native_options,
        Box::new(|cc| Box::new(MitmProxy::new(cc, listener_rx))),
    )
}