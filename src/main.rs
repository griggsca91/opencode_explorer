use color_eyre::Result;
use core::fmt::{self, Display};
use crossterm::event::{self, KeyCode, poll};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Row, Table, TableState};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use std::error::{self, Error};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Message {
    id: String,
    data: String,
}

struct OETableState {
    table_state: TableState,
    items: Vec<SessionRequestCount>,
}

struct SessionRequestCount {
    session_title: String,
    session_id: String,
    message_id: String,
    provider: String,
    model: String,
    count: i32,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Vec<SessionRequestCount>>();

    color_eyre::install()?;
    let mut table_state = TableState::default();
    table_state.select_first();
    table_state.select_first_column();

    let mut oe_table_state = OETableState {
        table_state: table_state,
        items: vec![],
    };
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let conn = rusqlite::Connection::open("/Users/chris/.local/share/opencode/opencode.db")
                .unwrap();
            let mut stmt = conn
                .prepare(
                    "
select s.title,
       m.session_id,
       m.data ->> 'providerID',
       m.data ->> 'modelID',
       coalesce(m.data ->> 'parentID', m.id) as message_id,
       count(1)
from message m
         join session s on m.session_id = s.id
where m.data ->> 'role' != 'user'
group by m.session_id, message_id, m.data ->> 'providerID', m.data ->> 'modelID'
order by max(s.time_updated) desc
limit 100;
",
                )
                .unwrap();
            let person_iter = stmt
                .query_map([], |row| {
                    Ok(SessionRequestCount {
                        session_title: row.get(0)?,
                        session_id: row.get(1)?,
                        provider: row.get(2)?,
                        model: row.get(3)?,
                        message_id: row.get(4)?,
                        count: row.get(5)?,
                    })
                })
                .unwrap()
                .map(|i| i.unwrap());

            tx.send(person_iter.collect()).unwrap();
        }
    });

    ratatui::run(|terminal| {
        loop {
            if let Ok(result) = rx.try_recv() {
                oe_table_state.items.clear();
                for person in result {
                    oe_table_state.items.push(person);
                }
            }
            terminal.draw(|frame| render(frame, &mut oe_table_state))?;

            if poll(Duration::from_millis(100))? {
                if let Some(key) = event::read()?.as_key_press_event() {
                    let mut table_state = oe_table_state.table_state;
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                        KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                        KeyCode::Char('l') | KeyCode::Right => table_state.select_next_column(),
                        KeyCode::Char('h') | KeyCode::Left => table_state.select_previous_column(),
                        KeyCode::Char('g') => table_state.select_first(),
                        KeyCode::Char('G') => table_state.select_last(),
                        _ => {}
                    };
                    oe_table_state.table_state = table_state;
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
    let header = Row::new(["Message ID", "Title", "Model", "Requests"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows = oe_table_state.items.iter().map(|i| {
        Row::new([
            i.message_id.clone(),
            i.session_title.clone(),
            format!("{} - {}", i.provider, i.model),
            i.count.to_string(),
        ])
    });

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, &mut oe_table_state.table_state);
}
