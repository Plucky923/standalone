mod home;

pub use home::draw_home;
mod language;
pub use language::draw_language;
mod bootstrap;
pub use bootstrap::draw_bootstrap;
mod sample_program;
pub use sample_program::draw_sample_program;
mod machine_mode;
pub use machine_mode::draw_machine_mode;
mod platform_support;
pub use platform_support::draw_platform_support;
mod allwinner_d1_series;
pub use allwinner_d1_series::draw_allwinner_d1_series;
mod sophgo_2002_series;
pub use sophgo_2002_series::draw_sophgo_2002_series;
mod standard_sbi_features;
pub use standard_sbi_features::draw_standard_sbi_features;
mod fdt_ident;
pub use fdt_ident::draw_fdt_ident;

use crate::{
    app::App,
    locale::{self, Translate},
};
use core::ops::ControlFlow;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, HighlightSpacing, Row, Table},
    Frame,
};

struct Builder {
    pub title: &'static str,
    pub header: Vec<&'static str>,
    pub items: Vec<Vec<&'static str>>,
    pub item_translate_idx: Vec<usize>,
    pub control_flow_fn: fn(usize, &mut App) -> ControlFlow<(), ()>,
    pub widths: Vec<Constraint>,
}

impl Builder {
    pub fn draw(self, f: &mut Frame, app: &mut App) {
        let mut items = self.items;
        for i in self.item_translate_idx {
            for row in &mut items {
                row[i] = row[i].translate(&app.locale);
            }
        }
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(0)
            .split(f.size());
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::LightBlue);

        let header = self.header.translate(&app.locale);
        let header_cells = header
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(0);
        let rows = items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16).bottom_margin(0)
        });
        let t = Table::new(rows, &self.widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(locale::get_string(self.title, &app.locale)),
            )
            .highlight_style(selected_style)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">> ");
        let state = &mut app.current_route_mut().table_state;
        f.render_stateful_widget(t, rects[0], state);

        app.item_length = items.len();
        app.control_flow_fn = Some(self.control_flow_fn);
    }
}
