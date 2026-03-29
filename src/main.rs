use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Row, Table, TableState};

#[derive(Debug)]
struct Message {
    id: String,
    data: String,
}

struct OETableState {
    table_state: TableState,
    items: Vec<Message>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut table_state = TableState::default();
    table_state.select_first();
    table_state.select_first_column();

    let mut oe_table_state = OETableState {
        table_state: table_state,
        items: vec![],
    };

    let conn = rusqlite::Connection::open("/Users/chris/.local/share/opencode/opencode.db")?;
    let mut stmt = conn.prepare("SELECT id, data FROM message limit 10")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            data: row.get(1)?,
        })
    })?;

    for person in person_iter {
        match person {
            Ok(m) => oe_table_state.items.push(m),
            _ => {}
        }
    }

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut oe_table_state))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                    KeyCode::Char('l') | KeyCode::Right => table_state.select_next_column(),
                    KeyCode::Char('h') | KeyCode::Left => table_state.select_previous_column(),
                    KeyCode::Char('g') => table_state.select_first(),
                    KeyCode::Char('G') => table_state.select_last(),
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, table_state: &mut OETableState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Table Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_table(frame, main, table_state);
}

/// Render a table with some rows and columns.
pub fn render_table(frame: &mut Frame, area: Rect, oe_table_state: &mut OETableState) {
    let header = Row::new(["Ingredient", "Quantity", "Macros"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows = oe_table_state
        .items
        .iter()
        .map(|i| Row::new([i.id.clone(), i.data.clone()]));

    let footer = Row::new([
        "Ratatouille Recipe",
        "",
        "135 kcal, 31g carbs, 6.4g protein",
    ]);
    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(50),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, &mut oe_table_state.table_state);
}
