//! Shared, bounded skill completion list. The invoking editor keeps focus and
//! owns selection; the scroll area contains only fixed-height candidate rows.
use crate::{input_bar::SkillEntry, ui_scale::ui_px};
use gpui::{
    App, Div, FontWeight, MouseButton, Pixels, Role, ScrollHandle, SharedString, Stateful, Window,
    div, prelude::*, px,
};
use gpui_component::{ActiveTheme, list::ListItem, scroll::ScrollableElement, tooltip::Tooltip};
use std::rc::Rc;

pub(crate) type ChooseSkill = Rc<dyn Fn(&str, &mut Window, &mut App)>;

pub(crate) fn skill_list(
    skills: &[SkillEntry],
    selected: usize,
    scroll: &ScrollHandle,
    max_height: Pixels,
    choose: ChooseSkill,
    cx: &App,
) -> Stateful<Div> {
    let theme = cx.theme();
    let secondary = theme.popover.blend(theme.foreground.opacity(0.72));
    let mut list = div()
        .id("skill-candidates")
        .role(Role::ListBox)
        .debug_selector(|| "skill-candidates".into())
        .w_full()
        .min_w_0()
        .min_h_0()
        .max_h(max_height)
        // Keep candidate mouse-down targets out of the overlaid thumb's lane.
        .pr(px(16.0))
        .flex()
        .flex_col()
        .overflow_y_scroll()
        .track_scroll(scroll)
        // The native scroll listener runs first; contain the wheel afterward
        // so overlapping terminal/editor scroll handlers cannot also consume it.
        .on_scroll_wheel(|_, _, cx| cx.stop_propagation());
    for (index, skill) in skills.iter().enumerate() {
        let name = skill.name.clone();
        let choose = choose.clone();
        let description = skill.description.clone();
        let preview = description.split_whitespace().collect::<Vec<_>>().join(" ");
        list = list.child(
            ListItem::new(SharedString::from(format!("skill-{name}")))
                .role(Role::ListBoxOption)
                .aria_selected(index == selected)
                .accessibility_label(format!("/{name}: {description}"))
                .debug_selector(move || format!("skill-candidate-{index}"))
                .selected(index == selected)
                .w_full()
                .min_w_0()
                .h(ui_px(theme, 48.0))
                .flex_none()
                .px(px(8.0))
                .py(px(6.0))
                .rounded(px(6.0))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    window.prevent_default();
                    cx.stop_propagation();
                    choose(&name, window, cx);
                })
                .tooltip(move |window, cx| Tooltip::new(description.clone()).build(window, cx))
                .child(
                    div()
                        .w_full()
                        .min_w_0()
                        .overflow_hidden()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .font_family(theme.mono_font_family.clone())
                                .text_size(ui_px(theme, 12.0))
                                .line_height(ui_px(theme, 18.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.foreground)
                                .child(format!("/{}", skill.name)),
                        )
                        .child(
                            div()
                                .truncate()
                                .font_family(theme.font_family.clone())
                                .text_size(ui_px(theme, 11.0))
                                .line_height(ui_px(theme, 16.0))
                                .text_color(secondary)
                                .child(preview),
                        ),
                ),
        );
    }
    // The scrollbar belongs to a fixed sibling layer, never to the content
    // whose offset it changes. Otherwise its track moves while it is dragged.
    div()
        .id("skill-completion-frame")
        .debug_selector(|| "skill-completion-frame".into())
        .relative()
        .w_full()
        .min_w_0()
        .min_h_0()
        .max_h(max_height)
        .overflow_hidden()
        .child(list)
        .vertical_scrollbar(scroll)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_bar::{InputBar, SkillAutocompleteChanged, SubmitInput};
    use gpui::{
        AppContext, Context, Entity, Focusable, IntoElement, Modifiers, MouseMoveEvent, Render,
        ScrollDelta, ScrollWheelEvent, point,
    };
    use std::cell::Cell;

    struct Fixture {
        bar: Entity<InputBar>,
        submits: Rc<Cell<usize>>,
        background_scrolls: Rc<Cell<usize>>,
    }
    impl Render for Fixture {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let skills = self
                .bar
                .read(cx)
                .filtered_skills(cx)
                .into_iter()
                .cloned()
                .collect::<Vec<_>>();
            let selected = self.bar.read(cx).skill_selection();
            let scroll = self.bar.read(cx).skill_popup_scroll_handle();
            let bar = self.bar.clone();
            let choose: ChooseSkill = Rc::new(move |name, window, cx| {
                bar.update(cx, |bar, cx| bar.complete_skill(name, window, cx))
            });
            let background_scrolls = self.background_scrolls.clone();
            div()
                .size_full()
                .on_scroll_wheel(move |_, _, _| {
                    background_scrolls.set(background_scrolls.get() + 1)
                })
                .child(
                    div()
                        .absolute()
                        .top(px(8.0))
                        .left(px(16.0))
                        .w(px(320.0))
                        .p(px(4.0))
                        .overflow_hidden()
                        .occlude()
                        .child(skill_list(
                            &skills,
                            selected,
                            &scroll,
                            px(120.0),
                            choose,
                            cx,
                        )),
                )
                .child(div().absolute().bottom_0().w_full().child(self.bar.clone()))
        }
    }
    #[gpui::test]
    fn long_descriptions_allow_keyboard_reveal_and_mouse_completion_past_third_item(
        cx: &mut gpui::TestAppContext,
    ) {
        con_core::release_channel::init();
        cx.update(gpui_component::init);
        cx.update(|cx| {
            gpui_component::Theme::set_scrollbar_mode(
                gpui_component::scroll::ScrollbarMode::Always,
                cx,
            )
        });
        cx.update(InputBar::init);
        let (view,cx) = cx.add_window_view(|window,cx| {
            let bar = cx.new(|cx| {
                let mut bar=InputBar::new(window,cx);
                bar.set_skills((0..8).map(|index|SkillEntry {
                    name:format!("skill-{index}"),
                    description:"Migrate Cloudflare Sandbox applications with a deliberately long description that previously expanded each candidate into a paragraph. ".repeat(4),
                }).collect(),cx);
                bar.focus_handle(cx).focus(window,cx);
                bar
            });
            cx.subscribe(&bar,|_,_,_:&SkillAutocompleteChanged,cx|cx.notify()).detach();
            let submits=Rc::new(Cell::new(0));
            let observed=submits.clone();
            cx.subscribe(&bar,move |_,_,_:&SubmitInput,_|observed.set(observed.get()+1)).detach();
            Fixture{bar,submits,background_scrolls:Rc::new(Cell::new(0))}
        });
        cx.simulate_input("/skill");
        for selected in 1..8 {
            cx.simulate_keystrokes("down");
            let scroll = view.read_with(cx, |view, cx| {
                assert_eq!(view.bar.read(cx).skill_selection(), selected);
                view.bar.read(cx).skill_popup_scroll_handle()
            });
            let viewport = scroll.bounds();
            let row = scroll.bounds_for_item(selected).expect("candidate bounds");
            let offset = scroll.offset();
            assert!(row.top() + offset.y >= viewport.top() - px(0.5));
            assert!(
                row.bottom() + offset.y <= viewport.bottom() + px(0.5),
                "selected candidate must be fully visible"
            );
            assert!(
                row.size.height <= px(48.5),
                "long prose must not grow completion rows"
            );
        }
        let scroll = view.read_with(cx, |view, cx| view.bar.read(cx).skill_popup_scroll_handle());
        // Wheel scrolling is independent of keyboard selection and must not
        // leak into the underlying editor, including inside an occluding popup.
        let position = scroll.bounds().center();
        cx.simulate_event(MouseMoveEvent {
            position,
            ..Default::default()
        });
        let before = scroll.offset().y;
        cx.simulate_event(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Pixels(point(px(0.0), px(96.0))),
            ..Default::default()
        });
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        assert!(
            scroll.offset().y > before,
            "wheel must move the candidates, not just keyboard reveal"
        );
        view.read_with(cx, |view, cx| {
            assert_eq!(
                view.bar.read(cx).skill_selection(),
                7,
                "wheel does not change the keyboard selection"
            );
            assert_eq!(
                view.background_scrolls.get(),
                0,
                "popup wheel must not reach the underlying editor"
            );
        });
        cx.simulate_event(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Lines(point(0.0, -20.0)),
            ..Default::default()
        });
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        let viewport = scroll.bounds();
        assert_eq!(cx.debug_bounds("skill-completion-frame").unwrap(), viewport);
        // Grab the visible fill, inside the library's 4pt outer inset.
        let thumb = point(viewport.right() - px(8.0), viewport.bottom() - px(12.0));
        let target = point(thumb.x, viewport.top() + px(8.0));
        let before_drag = scroll.offset().y;
        cx.update(|_, cx| {
            assert_eq!(
                cx.theme().scrollbar_mode,
                gpui_component::scroll::ScrollbarMode::Always
            );
            assert_eq!(
                gpui_base::Theme::global(cx).scrollbar.mode(),
                gpui_component::scroll::ScrollbarMode::Always
            );
        });
        cx.simulate_mouse_move(thumb, None, Modifiers::default());
        cx.run_until_parked();
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        cx.simulate_mouse_down(thumb, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        cx.simulate_mouse_move(target, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_up(target, MouseButton::Left, Modifiers::default());
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        assert!(
            scroll.offset().y > before_drag,
            "dragging the fixed scrollbar thumb must move the list: {viewport:?}, before {before_drag:?}, after {:?}",
            scroll.offset()
        );
        view.read_with(cx, |view, cx| {
            assert_eq!(
                view.bar.read(cx).current_text(cx),
                "/skill",
                "dragging the scrollbar must not choose a row"
            )
        });
        cx.simulate_keystrokes("up down");
        let last = scroll.bounds_for_item(7).unwrap();
        let offset = scroll.offset();
        cx.simulate_click(last.center() + offset, Modifiers::default());
        view.read_with(cx, |view, cx| {
            assert_eq!(view.bar.read(cx).current_text(cx), "/skill-7 ");
            assert!(view.bar.read(cx).filtered_skills(cx).is_empty());
            assert_eq!(
                view.submits.get(),
                0,
                "choosing a skill must not execute it"
            );
        });
        cx.update(|window, cx| {
            let bar = view.read(cx).bar.clone();
            assert!(bar.read(cx).focus_handle(cx).is_focused(window));
            bar.update(cx, |bar, cx| {
                bar.take_content(window, cx);
            });
        });
        cx.simulate_input("/skill");
        cx.simulate_keystrokes("down down down down down down down tab");
        view.read_with(cx, |view, cx| {
            assert_eq!(view.bar.read(cx).current_text(cx), "/skill-7 ")
        });
    }
}
