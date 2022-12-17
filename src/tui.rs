use gophersweeper::*;
use std::{
    error::Error,
    io::{stdout, Write}
};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    cursor,
    event::{self, Event, KeyCode},
    style::{Print, Color::{self, *}, SetForegroundColor, ResetColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};

const CELL:       &str = "■"; const CELL_COLOR:   Color = DarkGreen;
const FLAG:       &str = "!"; const FLAG_COLOR:   Color = DarkYellow;
const GOPHER:     &str = "@"; const GOPHER_COLOR: Color = Blue;
const EMPTY_CELL: &str = " ";

pub fn run(mut field: GopherSweeper) -> Result<(), Box<dyn Error>> {
    let (width, height, gophers) = (field.width, field.height, field.gophers);
    let (mut cursor_x, mut cursor_y) = (0, 0);

    terminal::enable_raw_mode()?;

    stdout()
        .queue(terminal::SetTitle("GopherSweeper"))?
        .queue(EnterAlternateScreen)?
        .queue(cursor::Show)?;

    draw_field(&field)?;
    stdout().queue(cursor::MoveTo(0, 0))?;
    stdout().flush()?;

    'a: loop {
        let cell = field.cell(cursor_x, cursor_y);

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                    if cursor_y > 0 {
                        stdout().execute(cursor::MoveUp(1))?;
                        cursor_y -= 1;
                    }
                },

                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                    if cursor_y < height - 1 {
                        stdout().execute(cursor::MoveDown(1))?;
                        cursor_y += 1;
                    }
                },

                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                    if cursor_x > 0 {
                        stdout().execute(cursor::MoveLeft(2))?;
                        cursor_x -= 1;
                    }
                },

                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                    if cursor_x < width - 1 {
                        stdout().execute(cursor::MoveRight(2))?;
                        cursor_x += 1;
                    }
                },

                KeyCode::Char(' ') => { 
                    if !cell.is_exposed && !cell.is_flagged {
                        match field.try_expose_cell(cursor_x, cursor_y) {
                            CellResult::Exposed => {
                                draw_field(&field)?;
                                stdout().flush()?;
                            },
    
                            CellResult::HasMine => {
                                stdout()
                                    .queue(SetForegroundColor(GOPHER_COLOR))?
                                    .queue(Print(GOPHER))?
                                    .queue(cursor::Hide)?
                                    .queue(cursor::MoveTo(0, height as u16))?
                                    .queue(SetForegroundColor(Red))?
                                    .queue(Print("You found a gopher :(\n"))?
                                    .queue(SetForegroundColor(Grey))?
                                    .queue(Print("\rPress <R> to restart or <ESC> to exit."))?
                                    .queue(ResetColor)?;

                                stdout().flush()?;

                                loop {
                                    if let Event::Key(key) = event::read()? {
                                        match key.code {
                                            KeyCode::Char('r') => {
                                                drop(field);
                                                run(GopherSweeper::new(width, height, gophers))?;
                                                break 'a
                                            },
    
                                            KeyCode::Esc => break 'a,
                                            _ => continue
                                        }
                                    }
                                }
                            },
    
                            CellResult::Win => {
                                stdout()
                                    .queue(SetForegroundColor(Grey))?
                                    .queue(Print(format!(
                                        "{} ",
                                        field.cell(cursor_x, cursor_y).surrounding_gophers
                                    )))?
                                    .queue(cursor::Hide)?
                                    .queue(cursor::MoveTo(0, height as u16))?
                                    .queue(SetForegroundColor(Green))?
                                    .queue(Print("Nice!\n"))?
                                    .queue(SetForegroundColor(Grey))?
                                    .queue(Print("\rPress a key to exit."))?
                                    .queue(ResetColor)?;

                                stdout().flush()?;

                                match event::read() { _ => break };
                            }
                        }
                    }
                },

                KeyCode::Char('f') => {
                    match (cell.is_exposed, cell.is_flagged) {
                        (false, false) => {
                            field.set_flag(cursor_x, cursor_y);

                            stdout()
                                .queue(cursor::SavePosition)?
                                .queue(SetForegroundColor(FLAG_COLOR))?
                                .queue(Print(FLAG))?
                                .queue(ResetColor)?
                                .queue(cursor::RestorePosition)?;

                            stdout().flush()?
                        },

                        (false, true) => {
                            field.set_flag(cursor_x, cursor_y);

                            stdout()
                                .queue(cursor::SavePosition)?
                                .queue(SetForegroundColor(CELL_COLOR))?
                                .queue(Print(CELL))?
                                .queue(ResetColor)?
                                .queue(cursor::RestorePosition)?;

                            stdout().flush()?
                        },

                        (true, _) => ()
                    }
                },

                _ => ()
            }
        }
    }

    stdout()
        .queue(cursor::Show)?
        .queue(LeaveAlternateScreen)?;
    
    stdout().flush()?;
    terminal::disable_raw_mode()?;

    Ok(())
}

 fn draw_field(field: &GopherSweeper) -> Result<(), Box<dyn Error>> {
    stdout()
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveTo(0, 0))?
        .queue(Clear(ClearType::All))?;

    for row in field {
        for cell in row {
            match (cell.is_exposed, cell.is_flagged) {
                (true, _) => {
                    match cell.surrounding_gophers {
                        0 => stdout()
                                .queue(SetForegroundColor(Grey))?
                                .queue(Print(format!("{EMPTY_CELL} ")))?,
                        n => {
                            stdout()
                                .queue(SetForegroundColor(Grey))?
                                .queue(Print(format!("{n} ")))?
                        }
                    }
                },

                (false, false) => {
                    stdout()
                        .queue(SetForegroundColor(DarkGreen))?
                        .queue(Print(format!("{CELL} ")))?
                },

                (false, true) => {
                    stdout()
                        .queue(SetForegroundColor(DarkYellow))?
                        .queue(Print(format!("{FLAG} ")))?
                }
            };
        }

        stdout().queue(Print("\r\n"))?;
    }

    stdout().queue(cursor::RestorePosition)?;
    Ok(())
}
