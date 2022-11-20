
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};

pub struct CheckBox<'a> {
    /// 文本
    text: String,
    /// 是否被激活，被激活显示为[*]，否则显示为[_]
    is_active: (bool, &'a str),
    /// 是否被选中
    is_picked: bool,
    /// 样式
    style: Style,
}

impl<'a> Default for CheckBox<'a> {
    fn default() -> Self {
        CheckBox {
            text: "null".to_string(),
            is_active: (false, "[_]"),
            is_picked: false,
            style: Style::default(),
        }
    }
}

impl<'a> CheckBox<'a> {
    pub fn new(text: &str) -> Self {
        CheckBox {
            text: text.to_string(),
            ..Default::default()
        }
    }
    pub fn text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn active(&mut self, is_active: bool) {
        self.is_active.0 = is_active;
        self.is_active.1 = Self::active_display(is_active);
    }

    pub fn picked(&mut self, is_picked: bool) {
        self.is_picked = is_picked;
    }

    pub fn style(&mut self, style: Style) {
        self.style = style;
    }

    fn active_display(is_active: bool) -> &'a str {
        if is_active {
            "[*]"
        } else {
            "[_]"
        }
    }
}

impl<'a> Widget for CheckBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.is_active.1.to_string() + &self.text, self.style);
    }
}
