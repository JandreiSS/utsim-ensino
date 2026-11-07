//! `utsim-ensino`: opens the window of UTSim Ensino.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use eframe::egui;
use utsim_ensino_store::caso;
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

/// Prints the result of a builder command (and writes it to `arquivo`, since the release
/// build on Windows has no console).
fn relatar(r: Result<String, utsim_ensino_store::Erro>, arquivo: Option<&Path>) -> ExitCode {
    match r {
        Ok(t) => {
            println!("{t}");
            if let Some(a) = arquivo {
                let _ = std::fs::write(a, &t);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("erro: {e}");
            if let Some(a) = arquivo {
                let _ = std::fs::write(a, format!("erro: {e}\n"));
            }
            ExitCode::FAILURE
        }
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
                "utsim-ensino [--missao <id>]\n  abre a janela do UTSim Ensino\n  --missao ato4/ventilador: abre essa missão direto (demonstração em aula)\n  UTSIM_ENSINO_PERFIL=<pasta>: guarda o perfil nessa pasta (computadores de laboratório)\n\nConstrutor de casos (o resultado também fica em <pasta>/conferencia.txt):\n  utsim-ensino caso <modelo.ron> <pasta> [título]: missão de rascunho a partir de um modelo\n  utsim-ensino conferir <pasta|pacote.utse>: confere as missões de um pacote\n  utsim-ensino pacote <pasta> <pacote.utse>: empacota a pasta"
            );
            return ExitCode::SUCCESS;
        }
        Some("caso") if args.len() >= 3 => {
            let titulo = args.get(3).cloned().unwrap_or_else(|| "Missão nova".into());
            return relatar(
                caso::criar_caso(Path::new(&args[1]), Path::new(&args[2]), &titulo),
                None,
            );
        }
        Some("conferir") if args.len() >= 2 => {
            let alvo = Path::new(&args[1]);
            let arquivo = alvo.is_dir().then(|| alvo.join("conferencia.txt"));
            return relatar(caso::conferir_pacote(alvo), arquivo.as_deref());
        }
        Some("pacote") if args.len() >= 3 => {
            return relatar(
                caso::empacotar(Path::new(&args[1]), Path::new(&args[2]))
                    .map(|p| format!("pacote gravado em {}", p.display())),
                None,
            );
        }
        // A builder command with missing arguments: never open the window by accident.
        Some("caso" | "conferir" | "pacote") => {
            eprintln!("faltam argumentos; veja utsim-ensino --ajuda");
            return ExitCode::FAILURE;
        }
        _ => {}
    }
    let missao = match args.first().map(String::as_str) {
        Some("--missao") => args.get(1).cloned(),
        _ => None,
    };
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
            let mut app = App::new(perfil);
            if let Some(id) = &missao
                && app.config.identidade.is_some()
                && let Some(i) = app.pacote.as_ref().and_then(|p| p.indice(id))
            {
                app.abrir_missao(i);
            }
            Ok(Box::new(Janela(app)))
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
