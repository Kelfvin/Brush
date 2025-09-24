use anyhow::{Ok, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    style::Stylize,
};
use ratatui::{
    DefaultTerminal, Frame,
    text::Line,
    widgets::{Block, Borders, List},
};

use crate::{
    database::{Database, book_dao::BookDao},
    models::book::Book,
};

#[derive(Debug)]
enum CurrenScreen {
    Practice(i64),         // question id
    PickSection(i64),      // section id
    PickBook(Option<i64>), // book id
}

impl Default for CurrenScreen {
    fn default() -> Self {
        CurrenScreen::PickBook(None)
    }
}

#[derive(Debug)]
pub struct APP {
    exit: bool,
    status: CurrenScreen,
    books: Vec<Book>,
    db: Database,
}

impl APP {
    pub async fn new() -> Self {
        let mut app = APP {
            exit: false,
            status: CurrenScreen::PickBook(None),
            books: vec![],
            db: Database::new("sqlite://brush.db")
                .await
                .expect("数据库无法打开"),
        };

        app.books = BookDao::select_all(&app.db.pool())
            .await
            .expect("无法加载Books");

        app
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        match self.status {
            CurrenScreen::Practice(_) => {}
            CurrenScreen::PickSection(_) => {}
            CurrenScreen::PickBook(_) => self.draw_pick_book_menu(frame),
        }
    }

    fn draw_pick_book_menu(&mut self, frame: &mut Frame) {
        let block = Block::default()
            .title("选择一本书".green().to_string())
            .borders(Borders::ALL);

        let list = List::new(
            self.books
                .iter()
                .map(|b| Line::from(b.name.clone()))
                .collect::<Vec<Line>>(),
        );

        frame.render_widget(list, frame.area());
    }

    fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(), // 退出
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn forward_manu(&mut self) {}

    fn backward_manu(&mut self) {}
}
