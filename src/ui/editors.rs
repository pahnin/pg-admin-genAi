use crate::ui::app_state::AppState;
use freya::prelude::*;

pub fn sql_editor_view(state: &mut AppState) -> Element {
  let mut focus_sql = state.focus_sql;
  let mut editable_sql = state.editable_sql;
  rsx!(
    rect {
      width: "calc(75%-10)",
      height: "100%",
      padding: "10",
      spacing: "10",
      corner_radius: "6",
      border: "0.3 inner black",
      label { "SQL:" }
      ScrollView {
        paragraph {
          width: "100%",
          height: "100%",
          cursor_id: "0",
          cursor_index: "{editable_sql.editor().read().cursor_pos()}",
          cursor_mode: "editable",
          cursor_color: "black",
          highlights: state.editable_sql.highlights_attr(0),
          cursor_reference: state.editable_sql.cursor_attr(),
          a11y_id: state.focus_sql.attribute(),
          onclick: move |_: Event<MouseData>| {
            focus_sql.request_focus();
            editable_sql.process_event(&EditableEvent::Click);
          },
          onmousemove: move |e: Event<MouseData>| {
            editable_sql.process_event(&EditableEvent::MouseMove(e.data, 0));
          },
          onmousedown: move |e: Event<MouseData>| {
            editable_sql.process_event(&EditableEvent::MouseDown(e.data, 0));
          },
          onkeydown: move |e: Event<KeyboardData>| {
            editable_sql.process_event(&EditableEvent::KeyDown(e.data));
          },
          onglobalkeyup: move |e: Event<KeyboardData>| {
            editable_sql.process_event(&EditableEvent::KeyUp(e.data));
          },
          text { "{editable_sql.editor()}" }
        }
      }
    }
  )
}

pub fn ai_chat_view(state: &mut AppState) -> Element {
  let mut focus_text = state.focus_text;
  let mut editable_nl = state.editable_nl;

  let conv = state.conversation.read();
  let mut scroll_controller = use_scroll_controller(|| ScrollConfig {
    default_vertical_position: ScrollPosition::End,
    ..Default::default()
  });
  use_effect(move || {
    scroll_controller.scroll_to(ScrollPosition::End, ScrollDirection::Vertical);
  });

  rsx!(
    rect {
      width: "25%",
      height: "100%",
      corner_radius: "6",
      border: "0.3 inner black",
      padding: "15 3",
      background: "rgb(233, 233, 233)",
      //
        rect {
          height: "75%",
          spacing: "7",
          ScrollView {
            scroll_controller: scroll_controller,
            for (i, msg) in conv.messages.iter().enumerate() {
              rect {
                key: "{i}",
                width: "100%",
                padding: "5",
                spacing: "2",
                corner_radius: "10",
                background: (if msg.role == "user" {
                  "rgb(240,240,255)"
                } else if msg.role == "assistant" {
                  "rgb(240,255,240)"
                } else {
                  "rgb(255,240,240)"
                }).to_string(),
                label {
                  font_size: "11",
                  font_weight: "bold",
                  "{msg.role}:"
                }
                label  {
                  font_size: "10",
                  font_weight: "light",
                  "{msg.content}"
                }
              }
            }
          }
        }

      rect {
        height: "25%",
        width: "100%",
        padding: "5",
        spacing: "3",
        background: "rgb(250,250,250)",
        corner_radius: "6",
        border: "0.3 inner black",
        label {
          font_size: "12",
          font_weight: "light",
          "Ask LLM:"
        },
        paragraph {
          width: "100%",
          height: "100%",
          cursor_id: "0",
          cursor_index: "{editable_nl.editor().read().cursor_pos()}",
          cursor_mode: "editable",
          cursor_color: "black",
          highlights: editable_nl.highlights_attr(0),
          cursor_reference: editable_nl.cursor_attr(),
          a11y_id: focus_text.attribute(),

          onclick: move |_: Event<MouseData>| {
              focus_text.request_focus();
              editable_nl.process_event(&EditableEvent::Click);
          },
          onmousemove: move |e: Event<MouseData>| {
              editable_nl.process_event(&EditableEvent::MouseMove(e.data, 0));
          },
          onmousedown: move |e: Event<MouseData>| {
              editable_nl.process_event(&EditableEvent::MouseDown(e.data, 0));
          },
          onkeydown: move |e: Event<KeyboardData>| {
              editable_nl.process_event(&EditableEvent::KeyDown(e.data));
          },
          onglobalkeyup: move |e: Event<KeyboardData>| {
              editable_nl.process_event(&EditableEvent::KeyUp(e.data));
          },

          text { "{editable_nl.editor()}" }
        }
      }
    }
  )
}
