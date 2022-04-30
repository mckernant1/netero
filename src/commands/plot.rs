use crate::args::plot::Plot;
use crate::utils::json_helper::JsonHelper;
use chrono::{NaiveDateTime, Utc};
use crossbeam::channel;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use log::debug;
use mckernant1_tools::crossbeam::stdin_reader;
use serde_json::Value;
use std::io;
use std::process::exit;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
use tui::text::Span;
use tui::widgets::{Axis, Chart, Dataset, GraphType};
use tui::Terminal;

#[derive(Clone, Debug, PartialEq)]
struct ScatterChart {
    lines: Vec<ScatterLine>,
}

impl ScatterChart {
    fn new(lines: Vec<ScatterLine>) -> ScatterChart {
        ScatterChart { lines }
    }

    fn max_datapoint(&self) -> i64 {
        self.lines
            .iter()
            .map(|it| it.max_datapoint())
            .max()
            .unwrap_or(0)
    }

    fn chart(&self, start_time: NaiveDateTime) -> Chart {
        let datasets = self
            .lines
            .iter()
            .map(|it| it.dataset())
            .collect::<Vec<Dataset>>();

        Chart::new(datasets)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .x_axis(
                Axis::default()
                    .bounds([start_time.timestamp() as f64, Utc::now().timestamp() as f64])
                    .labels(vec![
                        Span::styled(
                            start_time.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            Utc::now().naive_local().to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, self.max_datapoint() as f64])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(
                            self.max_datapoint().to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ]),
            )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ScatterLine {
    /// Name of the line
    key: String,
    /// Vec<timestamp, datapoint>
    datapoints: Vec<(f64, f64)>,
}

impl ScatterLine {
    fn new(name: String) -> ScatterLine {
        ScatterLine {
            key: name,
            datapoints: vec![],
        }
    }

    fn add_datapoint(&mut self, timestamp: i64, datapoint: f64) {
        self.datapoints.push((timestamp as f64, datapoint));
    }

    fn max_datapoint(&self) -> i64 {
        self.datapoints
            .iter()
            .map(|(_, v)| v.clone() as i64)
            .max()
            .unwrap_or(0)
    }

    fn dataset(&self) -> Dataset {
        Dataset::default()
            .name(&self.key)
            .marker(Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default())
            .data(&self.datapoints)
    }
}

pub fn plot(p: Plot) {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).unwrap();
    let (stdin_send, stdin_recv) = channel::unbounded::<Option<String>>();
    let stdin_thread = stdin_reader::read_stdin(stdin_send.clone());
    let start_timestamp = Utc::now().naive_local();

    let mut charts: Vec<ScatterChart> = p
        .plot_string
        .iter()
        .map(|it| {
            let lines = it
                .split("+")
                .map(|it| ScatterLine::new(it.to_string()))
                .collect::<Vec<ScatterLine>>();
            ScatterChart::new(lines)
        })
        .collect();

    while let Some(input) = stdin_recv.recv().unwrap() {
        let j = serde_json::from_str::<Value>(input.as_str()).unwrap();
        let input_timestamp = Utc::now().timestamp();

        for chart in charts.iter_mut() {
            chart.lines.iter_mut().for_each(|it| {
                let datapoint = match j.get_nested_attributes(it.key.clone()) {
                    Value::Number(n) => n.as_f64().unwrap(),
                    Value::String(s) => s.parse::<f64>().expect("Could not parse number from key"),
                    _ => {
                        eprintln!("Could not get value from key {}", it.key.clone());
                        exit(1);
                    }
                };
                it.add_datapoint(input_timestamp.clone(), datapoint);
            });
        }

        debug!("lines updated");
        term.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    charts
                        .iter()
                        .map(|_| Constraint::Ratio(1, charts.len() as u32))
                        .collect::<Vec<Constraint>>(),
                )
                .split(size);
            for (index, chart) in charts.iter().enumerate() {
                f.render_widget(chart.chart(start_timestamp), chunks[index]);
            }
        })
        .unwrap();
    }

    stdin_thread.join().unwrap();
    execute!(term.backend_mut(), LeaveAlternateScreen).unwrap();
}
