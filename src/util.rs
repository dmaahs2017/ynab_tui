use tui::layout::*;

pub fn milicent_to_dollars(amount: i64) -> f64 {
    amount as f64 / 1000.0
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn split_vertical(constraint: u16, area: Rect) -> (Rect, Rect) {
    let splits = Layout::default()
        .constraints([
            Constraint::Percentage(constraint),
            Constraint::Percentage(100 - constraint),
        ])
        .direction(Direction::Vertical)
        .split(area);
    (splits[0], splits[1])
}

pub fn master_stack_layout(children: u16, master_width: u16, area: Rect) -> (Rect, Vec<Rect>) {
    let stack_width = 100 - master_width;
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(stack_width),
            Constraint::Percentage(master_width),
        ])
        .direction(Direction::Horizontal)
        .split(area);

    let stack_area = chunks[0];
    let master_area = chunks[1];

    let stack_element_size = 100 / children;
    let stack_constraints: Vec<Constraint> = (0..children)
        .map(|_| Constraint::Percentage(stack_element_size))
        .collect();

    let stack = Layout::default()
        .constraints(stack_constraints)
        .direction(Direction::Vertical)
        .split(stack_area);

    (master_area, stack)
}
