use crate::ui::app_state::AppState;
use freya::prelude::*;

pub fn sql_editor_view(state: &mut AppState) -> Element {
  let mut focus_sql = state.focus_sql;
  let mut editable_sql = state.editable_sql;
  rsx!(
    rect {
      height: "calc(80% - 200)",
      padding: "10",
      spacing: "10",
      corner_radius: "6",
      border: "0.3 inner black",
      label { "SQL:" }
      ScrollView {
        paragraph {
          width: "100%",
          height: "100%",
          main_align: "center",
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

pub fn ai_input_editor_view(state: &mut AppState) -> Element {
  let mut focus_text = state.focus_text;
  let mut editable_nl = state.editable_nl;
  rsx!(
    rect {
      width: "100%",
      height: "120",
      corner_radius: "6",
      border: "0.3 inner black",
      padding: "10",
      spacing: "10",
      background: "rgb(233, 233, 233)",
      label { "Ai input:" }
      ScrollView {
        paragraph {
          width: "100%",
          height: "100%",
          main_align: "center",
          cursor_id: "0",
          cursor_index: "{editable_nl.editor().read().cursor_pos()}",
          cursor_mode: "editable",
          cursor_color: "black",
          highlights: state.editable_nl.highlights_attr(0),
          cursor_reference: state.editable_nl.cursor_attr(),
          a11y_id: state.focus_text.attribute(),
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
