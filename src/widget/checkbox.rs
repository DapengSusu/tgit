
use std::fmt::Display;

use tui::{
    style::Style, text::Span, widgets::Widget,
};

pub struct CheckBox {
    /// 头部，显示为[*]或[_]，默认显示[_]
    icon: String,
    /// 文本
    text: String,
    /// 头部左上角位置
    pos: (f64, f64),
    /// 是否被激活，被激活显示为[*]，否则显示为[_]
    is_active: bool,
    /// 是否被选中
    is_picked: bool,
    /// 样式
    style: Style,
}

impl Default for CheckBox {
    fn default() -> CheckBox {
        CheckBox {
            icon: "[_]".to_string(),
            text: "null checkbox".to_string(),
            pos: (0_f64, 0_f64),
            is_active: false,
            is_picked: false,
            style: Style::default(),
        }
    }
}

impl CheckBox {
    pub fn new(text: String) -> CheckBox {
        CheckBox {
            text,
            ..Default::default()
        }
    }

    pub fn text(mut self, text: String) -> CheckBox {
        self.text = text;

        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> CheckBox {
        self.pos.0 = x;
        self.pos.1 = y;

        self
    }

    pub fn active(mut self, is_active: bool) -> CheckBox {
        self.is_active = is_active;
        self.icon = Self::icon(is_active);

        self
    }

    pub fn picked(mut self, is_picked: bool) -> CheckBox {
        self.is_picked = is_picked;

        self
    }

    pub fn style(mut self, style: Style) -> CheckBox {
        self.style = style;

        self
    }

    pub fn update_icon(&mut self, is_active: bool) {
        self.is_active = is_active;
        self.icon = Self::icon(is_active);
    }

    fn icon(is_active: bool) -> String{
        if is_active {
            "[*]".to_string()
        } else {
            "[_]".to_string()
        }
    }
}

impl Display for CheckBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.icon, self.text)
    }
}

impl<'a> From<Span<'a>> for CheckBox {
    fn from(s: Span<'a>) -> Self {
        CheckBox::default().text(String::from(s.content))
    }
}

impl Widget for CheckBox {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        buf.set_string(area.left(), area.top(), self.text, self.style);
    }
}
