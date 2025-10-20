mod actions;
mod app;
mod components;
mod models;
mod selectors;
mod states;
mod task_repo;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal);
    ratatui::restore();
    result
}
