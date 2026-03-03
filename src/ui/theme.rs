use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Theme {
    pub name: String,
    pub user_message: Style,
    pub assistant_message: Style,
    pub system_message: Style,
    pub tool_pending: Color,
    pub tool_running: Color,
    pub tool_success: Color,
    pub tool_error: Color,
    pub code_block_bg: Color,
    pub header_bg: Color,
    pub status_bar_bg: Color,
    pub input_border: Color,
    pub separator: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            user_message: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::White),
            system_message: Style::default().fg(Color::Yellow),
            tool_pending: Color::Yellow,
            tool_running: Color::Cyan,
            tool_success: Color::Green,
            tool_error: Color::Red,
            code_block_bg: Color::Rgb(40, 42, 54),
            header_bg: Color::Rgb(30, 30, 46),
            status_bar_bg: Color::Rgb(30, 30, 46),
            input_border: Color::Cyan,
            separator: Color::DarkGray,
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            user_message: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::Black),
            system_message: Style::default().fg(Color::Rgb(180, 100, 0)),
            tool_pending: Color::Rgb(200, 150, 0),
            tool_running: Color::Blue,
            tool_success: Color::Green,
            tool_error: Color::Red,
            code_block_bg: Color::Rgb(245, 245, 245),
            header_bg: Color::Rgb(230, 230, 230),
            status_bar_bg: Color::Rgb(230, 230, 230),
            input_border: Color::Blue,
            separator: Color::Gray,
        }
    }

    pub fn monokai() -> Self {
        Self {
            name: "monokai".to_string(),
            user_message: Style::default().fg(Color::Rgb(102, 217, 239)).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::Rgb(248, 248, 242)),
            system_message: Style::default().fg(Color::Rgb(230, 219, 116)),
            tool_pending: Color::Rgb(230, 219, 116),
            tool_running: Color::Rgb(102, 217, 239),
            tool_success: Color::Rgb(166, 226, 46),
            tool_error: Color::Rgb(249, 38, 114),
            code_block_bg: Color::Rgb(39, 40, 34),
            header_bg: Color::Rgb(39, 40, 34),
            status_bar_bg: Color::Rgb(39, 40, 34),
            input_border: Color::Rgb(102, 217, 239),
            separator: Color::Rgb(117, 113, 94),
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            user_message: Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::Rgb(248, 248, 242)),
            system_message: Style::default().fg(Color::Rgb(241, 250, 140)),
            tool_pending: Color::Rgb(241, 250, 140),
            tool_running: Color::Rgb(139, 233, 253),
            tool_success: Color::Rgb(80, 250, 123),
            tool_error: Color::Rgb(255, 85, 85),
            code_block_bg: Color::Rgb(40, 42, 54),
            header_bg: Color::Rgb(40, 42, 54),
            status_bar_bg: Color::Rgb(40, 42, 54),
            input_border: Color::Rgb(189, 147, 249),
            separator: Color::Rgb(68, 71, 90),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            user_message: Style::default().fg(Color::Rgb(136, 192, 208)).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::Rgb(236, 239, 244)),
            system_message: Style::default().fg(Color::Rgb(235, 203, 139)),
            tool_pending: Color::Rgb(235, 203, 139),
            tool_running: Color::Rgb(136, 192, 208),
            tool_success: Color::Rgb(163, 190, 140),
            tool_error: Color::Rgb(191, 97, 106),
            code_block_bg: Color::Rgb(46, 52, 64),
            header_bg: Color::Rgb(46, 52, 64),
            status_bar_bg: Color::Rgb(46, 52, 64),
            input_border: Color::Rgb(129, 161, 193),
            separator: Color::Rgb(76, 86, 106),
        }
    }

    pub fn solarized() -> Self {
        Self {
            name: "solarized".to_string(),
            user_message: Style::default().fg(Color::Rgb(38, 139, 210)).add_modifier(Modifier::BOLD),
            assistant_message: Style::default().fg(Color::Rgb(131, 148, 150)),
            system_message: Style::default().fg(Color::Rgb(181, 137, 0)),
            tool_pending: Color::Rgb(181, 137, 0),
            tool_running: Color::Rgb(38, 139, 210),
            tool_success: Color::Rgb(133, 153, 0),
            tool_error: Color::Rgb(220, 50, 47),
            code_block_bg: Color::Rgb(0, 43, 54),
            header_bg: Color::Rgb(0, 43, 54),
            status_bar_bg: Color::Rgb(0, 43, 54),
            input_border: Color::Rgb(42, 161, 152),
            separator: Color::Rgb(88, 110, 117),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "light" => Self::light(),
            "monokai" => Self::monokai(),
            "dracula" => Self::dracula(),
            "nord" => Self::nord(),
            "solarized" => Self::solarized(),
            _ => Self::dark(),
        }
    }

    pub fn available_themes() -> Vec<&'static str> {
        vec!["dark", "light", "monokai", "dracula", "nord", "solarized"]
    }
}
