use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    event::{
        self, Event, KeyCode, KeyEvent,
        KeyEventKind::{Press, Repeat},
        KeyModifiers,
    },
    style::{Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::application::{Application, Menu, MenuUpdate};

impl<T: Write> Application<T> {
    pub(in crate::application) fn run_menu_about(&mut self) -> io::Result<MenuUpdate> {
        let mut scroll: usize = 0;
        loop {
            let w_main: usize = Self::W_MAIN.into();
            let (x_main, y_main) = Self::fetch_main_xy();
            let y_start = y_main + 1;

            let lines: Vec<String> = vec![
                format!("{:^w_main$}", format!("- About Tetro TUI {} -", clap::crate_version!())),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "https://github.com/Strophox/tetro-tui"),
                format!("{:^w_main$}", "A terminal-based falling-block stacking game."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "────────── Controls ──────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "[Esc] Pause    [Ctrl+D] Forfeit"),
                format!("{:^w_main$}", "[Ctrl+S] Store savepoint"),
                format!("{:^w_main$}", "[Ctrl+E] Store seed"),
                format!("{:^w_main$}", "[Ctrl+Alt+B] Toggle blindfold"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "────────── Pieces ──────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "I ▄▄▄▄    O ██    T ▄█▄"),
                format!("{:^w_main$}", "S ▄█▀     Z ▀█▄"),
                format!("{:^w_main$}", "L ▄▄█     J █▄▄"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "All 7 standard tetrominoes, drawn from"),
                format!("{:^w_main$}", "a shuffled bag (each appears once per"),
                format!("{:^w_main$}", "bag of 7). Hold a piece with [Hold]."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "──────── Game Modes ────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "40-Lines  Clear 40 lines as fast as"),
                format!("{:^w_main$}", "          you can. Best: lowest time."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Marathon  Clear 150 lines with"),
                format!("{:^w_main$}", "          increasing gravity."),
                format!("{:^w_main$}", "          Best: highest score."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Time Trial  3 minutes, constant"),
                format!("{:^w_main$}", "            gravity. Score as much"),
                format!("{:^w_main$}", "            as possible."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Master    Instant gravity (20G)."),
                format!("{:^w_main$}", "          Pieces land immediately."),
                format!("{:^w_main$}", "          150 lines to clear."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Puzzle    Pre-set board layouts."),
                format!("{:^w_main$}", "          Clear them all!"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Cheese    Board filled with garbage"),
                format!("{:^w_main$}", "          lines. Dig through them"),
                format!("{:^w_main$}", "          using the fewest pieces."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Combo     Pre-set board for chaining"),
                format!("{:^w_main$}", "          consecutive line clears."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Custom    Configure your own rules:"),
                format!("{:^w_main$}", "          gravity, win condition,"),
                format!("{:^w_main$}", "          seed, and starting board."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "────────── Scoring ──────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Score = (Perfect?4:1) * (Spin?2:1)"),
                format!("{:^w_main$}", "      * (Lines*2 - 1) + (Combo - 1)"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "\"Perfect\" = board fully empty after"),
                format!("{:^w_main$}", "the line clear (4x multiplier)."),
                format!("{:^w_main$}", "All spins count, not just T-spins."),
                format!("{:^w_main$}", "Combos add bonus points per chain."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "──────── Glossary ────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "PPS   Pieces Per Second - how fast"),
                format!("{:^w_main$}", "      you place pieces."),
                format!("{:^w_main$}", "DAS   Delayed Auto Shift - delay"),
                format!("{:^w_main$}", "      before held key auto-repeats."),
                format!("{:^w_main$}", "ARR   Auto Repeat Rate - speed of"),
                format!("{:^w_main$}", "      auto-repeat once DAS fires."),
                format!("{:^w_main$}", "20G   Instant gravity. Pieces fall"),
                format!("{:^w_main$}", "      to the bottom immediately."),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "────────── Credits ──────────"),
                format!("{:^w_main$}", ""),
                format!("{:^w_main$}", "Created by Strophox"),
                format!("{:^w_main$}", "License: MIT"),
            ];

            let visible_lines = usize::from(Self::H_MAIN).saturating_sub(2);
            let max_scroll = lines.len().saturating_sub(visible_lines);
            scroll = scroll.min(max_scroll);

            self.term.queue(Clear(ClearType::All))?;

            for (i, line) in lines.iter().skip(scroll).take(visible_lines).enumerate() {
                self.term
                    .queue(MoveTo(x_main, y_start + u16::try_from(i).unwrap()))?
                    .queue(Print(line))?;
            }

            if max_scroll > 0 {
                self.term
                    .queue(MoveTo(x_main, y_main + Self::H_MAIN - 1))?
                    .queue(PrintStyledContent(
                        format!("{:^w_main$}", "[Up/Down to scroll, Esc to return]").italic(),
                    ))?;
            } else {
                self.term
                    .queue(MoveTo(x_main, y_main + Self::H_MAIN - 1))?
                    .queue(PrintStyledContent(
                        format!("{:^w_main$}", "[Esc to return]").italic(),
                    ))?;
            }

            self.term.flush()?;

            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c' | 'C'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press | Repeat,
                    state: _,
                }) => break Ok(MenuUpdate::Push(Menu::Quit)),
                Event::Key(KeyEvent {
                    code:
                        KeyCode::Esc
                        | KeyCode::Char('q' | 'Q')
                        | KeyCode::Backspace
                        | KeyCode::Char('b' | 'B'),
                    kind: Press,
                    ..
                }) => break Ok(MenuUpdate::Pop),
                Event::Key(KeyEvent {
                    code: KeyCode::Up | KeyCode::Char('k' | 'K'),
                    kind: Press | Repeat,
                    ..
                }) => {
                    scroll = scroll.saturating_sub(1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down | KeyCode::Char('j' | 'J'),
                    kind: Press | Repeat,
                    ..
                }) => {
                    scroll = scroll.saturating_add(1);
                }
                _ => {}
            }
        }
    }
}
