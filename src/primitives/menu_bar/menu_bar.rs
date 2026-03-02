use crate::primitives::menu_bar::util::display_width;
use crate::primitives::widget_event::WidgetEvent;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
#[cfg(feature = "theme")]
use ratkit_theme::AppTheme;

pub struct MenuItem {
    pub name: String,
    pub icon: Option<String>,
    pub value: usize,
    pub selected: bool,
    pub hovered: bool,
    pub area: Option<Rect>,
    pub action: Option<Box<dyn FnOnce() + Send>>,
}

impl MenuItem {
    pub fn new(name: impl Into<String>, value: usize) -> Self {
        Self {
            name: name.into(),
            icon: None,
            value,
            selected: false,
            hovered: false,
            area: None,
            action: None,
        }
    }

    pub fn with_icon(name: impl Into<String>, icon: impl Into<String>, value: usize) -> Self {
        Self {
            name: name.into(),
            icon: Some(icon.into()),
            value,
            selected: false,
            hovered: false,
            area: None,
            action: None,
        }
    }

    pub fn with_action(
        name: impl Into<String>,
        value: usize,
        action: impl FnOnce() + Send + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            icon: None,
            value,
            selected: false,
            hovered: false,
            area: None,
            action: Some(Box::new(action)),
        }
    }

    pub fn with_icon_and_action(
        name: impl Into<String>,
        icon: impl Into<String>,
        value: usize,
        action: impl FnOnce() + Send + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            icon: Some(icon.into()),
            value,
            selected: false,
            hovered: false,
            area: None,
            action: Some(Box::new(action)),
        }
    }

    pub fn display_label(&self) -> String {
        if let Some(ref icon) = self.icon {
            format!("{} {}", icon, self.name)
        } else {
            self.name.clone()
        }
    }
}

pub struct MenuBar {
    pub items: Vec<MenuItem>,
    pub area: Option<Rect>,
    pub normal_style: Style,
    pub selected_style: Style,
    pub hover_style: Style,
    pub selected_hover_style: Style,
}

impl MenuBar {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            area: None,
            normal_style: Style::default().fg(Color::White),
            selected_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            hover_style: Style::default().fg(Color::Cyan),
            selected_hover_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.items[index].selected = true;
        }
        self
    }

    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    pub fn selected_hover_style(mut self, style: Style) -> Self {
        self.selected_hover_style = style;
        self
    }

    #[cfg(feature = "theme")]
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.normal_style = Style::default().fg(theme.text);
        self.selected_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.hover_style = Style::default().fg(theme.secondary);
        self.selected_hover_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self
    }

    #[cfg(feature = "theme")]
    pub fn apply_theme(&mut self, theme: &AppTheme) {
        self.normal_style = Style::default().fg(theme.text);
        self.selected_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.hover_style = Style::default().fg(theme.secondary);
        self.selected_hover_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
    }

    pub fn update_hover(&mut self, column: u16, row: u16) {
        for item in &mut self.items {
            item.hovered = if let Some(area) = item.area {
                column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
            } else {
                false
            };
        }
    }

    pub fn handle_click(&mut self, column: u16, row: u16) -> Option<usize> {
        let clicked_index = self.items.iter().enumerate().find_map(|(i, item)| {
            if let Some(area) = item.area {
                if column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    return Some(i);
                }
            }
            None
        });

        if let Some(clicked) = clicked_index {
            for (i, item) in self.items.iter_mut().enumerate() {
                item.selected = i == clicked;
            }
        }

        clicked_index
    }

    pub fn handle_mouse(&mut self, column: u16, row: u16) -> WidgetEvent {
        let clicked_index = self.items.iter().enumerate().find_map(|(i, item)| {
            if let Some(area) = item.area {
                if column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    return Some(i);
                }
            }
            None
        });

        if let Some(clicked) = clicked_index {
            let action = self.items[clicked].action.take();
            for (i, item) in self.items.iter_mut().enumerate() {
                item.selected = i == clicked;
            }
            return WidgetEvent::MenuSelected {
                index: clicked,
                action,
            };
        }

        WidgetEvent::None
    }

    pub fn selected(&self) -> Option<usize> {
        self.items.iter().position(|item| item.selected)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.render_with_offset(frame, area, 0);
    }

    pub fn render_with_offset(&mut self, frame: &mut Frame, area: Rect, left_offset: u16) {
        if self.items.is_empty() {
            return;
        }

        let available_width = area.width.saturating_sub(left_offset);
        if available_width == 0 {
            self.area = None;
            return;
        }

        let button_group_area = Rect {
            x: area.x + left_offset,
            y: area.y,
            width: available_width,
            height: area.height,
        };

        self.area = Some(button_group_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(button_group_area);
        frame.render_widget(block, button_group_area);

        let mut x_offset = inner_area.x + 1;
        let button_count = self.items.len();

        for (i, item) in self.items.iter_mut().enumerate() {
            let label = item.display_label();
            let item_width = display_width(&label) as u16;

            let available_width = (inner_area.x + inner_area.width).saturating_sub(x_offset);
            if available_width == 0 {
                break;
            }

            let actual_item_width = item_width.min(available_width);

            let item_area = Rect {
                x: x_offset,
                y: inner_area.y,
                width: actual_item_width,
                height: inner_area.height,
            };

            item.area = Some(item_area);

            let style = match (item.selected, item.hovered) {
                (true, true) => self.selected_hover_style,
                (true, false) => self.selected_style,
                (false, true) => self.hover_style,
                (false, false) => self.normal_style,
            };

            let display_label = if actual_item_width < item_width {
                label
                    .chars()
                    .take(actual_item_width as usize)
                    .collect::<String>()
            } else {
                label
            };
            let paragraph = Paragraph::new(display_label).style(style);
            frame.render_widget(paragraph, item_area);

            x_offset += actual_item_width;

            if i < button_count - 1 && x_offset + 3 <= inner_area.x + inner_area.width {
                let separator_area = Rect {
                    x: x_offset,
                    y: inner_area.y,
                    width: 3,
                    height: inner_area.height,
                };
                let separator = Paragraph::new(" │ ");
                frame.render_widget(separator, separator_area);
                x_offset += 3;
            }
        }
    }

    pub fn render_centered(&mut self, frame: &mut Frame, area: Rect) {
        let total_chars: usize = self
            .items
            .iter()
            .map(|item| display_width(&item.display_label()) + 4)
            .sum();
        let needed_width = total_chars as u16;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((area.width.saturating_sub(needed_width)) / 2),
                Constraint::Length(needed_width.min(area.width)),
                Constraint::Min(0),
            ])
            .split(area);

        self.render(frame, chunks[1]);
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new(vec![MenuItem::new("Menu Item", 0)])
    }
}
