//! `utsim-ensino`: opens the window of UTSim Ensino.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;
use std::sync::Arc;

use eframe::egui;
use utsim_ensino_store::perfil::Perfil;
use utsim_ensino_ui::App;
use utsim_ensino_ui::texto::NOME_APP;

/// The icon of the window and the taskbar.
const ICONE: &[u8] = include_bytes!("../../../assets/icone/familia/ensino_256.png");

struct Janela(App);

impl eframe::App for Janela {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.0.draw(ui);
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--versao" | "--version" | "-V") => {
            println!("utsim-ensino {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
        Some("--ajuda" | "--help" | "-h") => {
            println!(
                "utsim-ensino\n  abre a janela do UTSim Ensino\n  UTSIM_ENSINO_PERFIL=<pasta>: guarda o perfil nessa pasta (computadores de laboratório)"
            );
            return ExitCode::SUCCESS;
        }
        _ => {}
    }
    let Some(perfil) = Perfil::padrao() else {
        eprintln!("erro: não encontrei a pasta do usuário");
        return ExitCode::FAILURE;
    };
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(NOME_APP)
        .with_inner_size([1180.0, 800.0])
        .with_min_inner_size([760.0, 480.0]);
    if let Some((w, h, rgba)) = utsim_ui_tema::familia::png_rgba(ICONE) {
        viewport = viewport.with_icon(Arc::new(egui::IconData {
            rgba,
            width: w as u32,
            height: h as u32,
        }));
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    let result = eframe::run_native(
        NOME_APP,
        options,
        Box::new(move |cc| {
            cc.egui_ctx
                .set_pixels_per_point(cc.egui_ctx.pixels_per_point().max(1.1));
            Ok(Box::new(Janela(App::new(perfil))))
        }),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erro na janela: {e}");
            ExitCode::FAILURE
        }
    }
}
