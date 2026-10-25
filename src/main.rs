use chrono::{Datelike, Local, NaiveDate};
use freya::prelude::*;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const BASE: (u8, u8, u8) = (0x1e, 0x1e, 0x2e);
const SURFACE0: (u8, u8, u8) = (0x31, 0x32, 0x44);
const SURFACE1: (u8, u8, u8) = (0x45, 0x47, 0x5a);
const SURFACE2: (u8, u8, u8) = (0x58, 0x5b, 0x70);
const TEXT: (u8, u8, u8) = (0xcd, 0xd6, 0xf4);
const BLUE: (u8, u8, u8) = (0x89, 0xb4, 0xfa);
const RED: (u8, u8, u8) = (0xf3, 0x8b, 0xa8);
const PINK: (u8, u8, u8) = (0xf5, 0xc2, 0xe7);
const OVERLAY0: (u8, u8, u8) = (0x6c, 0x70, 0x86);

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Simple Calendar")
                .with_size(440., 520.)
                .with_background(BASE),
        ),
    )
}

fn app() -> impl IntoElement {
    let now = Local::now();
    let mut year = use_state(|| now.year());
    let mut month = use_state(|| now.month());
    let mut year_text = use_state(|| now.year().to_string());
    let selected = use_state(|| None::<NaiveDate>);
    let mut month_text = use_state(|| MONTH_NAMES[now.month0() as usize].to_string());

    let mut set_view = move |new_year: i32, new_month: u32| {
        year.set(new_year);
        month.set(new_month);
        year_text.set(new_year.to_string());
        month_text.set(MONTH_NAMES[(new_month - 1) as usize].to_string());
    };

    let mut prev_month = move || {
        if month() == 1 {
            set_view(year() - 1, 12)
        } else {
            set_view(year(), month() - 1)
        }
    };
    let mut next_month = move || {
        if month() == 12 {
            set_view(year() + 1, 1)
        } else {
            set_view(year(), month() + 1)
        }
    };
    let mut prev_year = move || set_view(year() - 1, month());
    let mut next_year = move || set_view(year() + 1, month());

    let on_year_submit = move |text: String| {
        match text.trim().parse::<i32>() {
            Ok(y) if y > 0 && y < 10000 => set_view(y, month()),
            _ => set_view(year(), month()),
        }
    };
    let on_month_submit = move |text: String| {
        let text = text.trim().to_lowercase();
        let parsed = text
            .parse::<u32>()
            .ok()
            .filter(|m| (1..=12).contains(m))
            .or_else(|| {
                MONTH_NAMES
                    .iter()
                    .position(|name| name.to_lowercase() == text)
                    .map(|idx| idx as u32 + 1)
            });
        set_view(year(), parsed.unwrap_or(month()));
    };

    rect()
        .expanded()
        .background(BASE)
        .vertical()
        .main_align(Alignment::Center)
        .cross_align(Alignment::Center)
        .spacing(8.)
        .padding(12.)
        .on_global_key_down(move |e: Event<KeyboardEventData>| match e.key {
            Key::Named(NamedKey::ArrowLeft) => prev_month(),
            Key::Named(NamedKey::ArrowRight) => next_month(),
            Key::Named(NamedKey::ArrowUp) => next_year(),
            Key::Named(NamedKey::ArrowDown) => prev_year(),
            _ => {}
        })
        .child(
            nav_row(8.)
                .child(nav_button("‹", SURFACE0, SURFACE1, 20., move || prev_year()))
                .child(
                    nav_input(year_text, 80., SURFACE0, SURFACE2).on_submit(on_year_submit),
                )
                .child(nav_button("›", SURFACE0, SURFACE1, 20., move || next_year())),
        )
        .child(
            nav_row(16.)
                .child(nav_button("‹", SURFACE1, SURFACE2, 18., move || prev_month()))
                .child(
                    nav_input(month_text, 120., SURFACE1, SURFACE2).on_submit(on_month_submit),
                )
                .child(nav_button("›", SURFACE1, SURFACE2, 18., move || next_month())),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .children(DAY_NAMES.iter().map(|name| -> Element {
                    rect()
                        .width(Size::px(48.))
                        .height(Size::px(32.))
                        .center()
                        .child(
                            label()
                                .text(*name)
                                .font_size(12.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .font_family("monospace")
                                .color(BLUE),
                        )
                        .into()
                })),
        )
        .child(calendar_grid(year(), month(), selected))
        .child(
            label()
                .text(match selected() {
                    Some(date) => date.format("%A, %B %-d, %Y").to_string(),
                    None => "←→ Months  ↑↓ Years".to_string(),
                })
                .font_size(15.)
                .font_family("monospace")
                .color(OVERLAY0),
        )
}

fn nav_row(spacing: f32) -> Rect {
    rect()
        .horizontal()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .spacing(spacing)
}

fn nav_button(
    glyph: &'static str,
    bg: (u8, u8, u8),
    border: (u8, u8, u8),
    size: f32,
    mut on_press: impl FnMut() + 'static,
) -> Rect {
    rect()
        .width(Size::px(40.))
        .height(Size::px(36.))
        .center()
        .background(bg)
        .corner_radius(6.)
        .border(Border::new().fill(border).width(1.))
        .on_press(move |_| on_press())
        .child(label().text(glyph).font_size(size).color(BLUE))
}

fn nav_input(value: State<String>, width: f32, bg: (u8, u8, u8), border: (u8, u8, u8)) -> Input {
    Input::new(value)
        .width(Size::px(width))
        .text_align(TextAlign::Center)
        .theme_colors(InputColorsThemePartial {
            background: Some(Color::from(bg).into()),
            focus_background: Some(Color::from(bg).into()),
            border_fill: Some(Color::from(border).into()),
            focus_border_fill: Some(Color::from(BLUE).into()),
            color: Some(Color::from(TEXT).into()),
            ..Default::default()
        })
}

fn calendar_grid(
    year: i32,
    month: u32,
    mut selected: State<Option<NaiveDate>>,
) -> Rect {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let start = first.weekday().num_days_from_sunday();
    let days = days_in_month(year, month);
    let today = Local::now();
    let today_day = (year == today.year() && month == today.month()).then(|| today.day());

    // 7 rows x 7 columns, like the original layout.
    rect().vertical().spacing(2.).children((0..7u32).map(move |row| -> Element {
        rect()
            .horizontal()
            .spacing(8.)
            .children((0..7u32).map(move |col| -> Element {
                let idx = row * 7 + col;
                let day = idx
                    .checked_sub(start)
                    .map(|d| d + 1)
                    .filter(|d| *d <= days);
                let is_today = day.is_some() && day == today_day;
                let date = day.and_then(|d| NaiveDate::from_ymd_opt(year, month, d));
                let is_selected = date.is_some() && date == selected();
                day_cell(day, is_today, is_selected)
                    .on_press(move |_| {
                        if let Some(date) = date {
                            // clicking the selected day again clears the selection
                            selected.set((selected() != Some(date)).then_some(date));
                        }
                    })
                    .into()
            }))
            .into()
    }))
}

fn day_cell(day: Option<u32>, is_today: bool, is_selected: bool) -> Rect {
    let cell = rect()
        .width(Size::px(48.))
        .height(Size::px(40.))
        .center()
        .corner_radius(8.);
    let Some(day) = day else { return cell };
    let cell = if is_today {
        cell.background(RED)
            .border(Border::new().fill(PINK).width(2.))
    } else if is_selected {
        cell.background(SURFACE1)
            .border(Border::new().fill(BLUE).width(2.))
    } else {
        cell.background(SURFACE0)
    };
    cell.child(
        label()
            .text(day.to_string())
            .font_size(14.)
            .font_family("monospace")
            .font_weight(if is_today {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            })
            .color(if is_today { BASE } else { TEXT }),
    )
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day()
}
