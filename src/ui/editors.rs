use crate::ui::app_state::AppState;
use freya::prelude::*;

pub fn sql_editor_view(state: &mut AppState) -> Element {
  let mut focus_sql = state.focus_sql;
  let mut editable_sql = state.editable_sql;
  rsx!(
    rect {
      width: "calc(80%-10)",
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
/*  let conv_res = use_resource(move || async move {*/
    /*let conv = state.conversation.read().await;*/
    /*conv.to_owned()*/
  /*});*/

    let conv = state.conversation.blocking_read();

  rsx!(
    rect {
      width: "20%",
      height: "100%",
      corner_radius: "6",
      border: "0.3 inner black",
      padding: "10",
      background: "rgb(233, 233, 233)",
      //flex_direction: "column",
      //
        rect {
          height: "75%",
          ScrollView {
            // directly iterate inside rsx! — do NOT call rsx! inside the loop
            for (i, msg) in conv.messages.iter().enumerate() {
              rect {
                key: format!("msg-{}", i), // avoid "{i}" ambiguity
                width: "100%",
                padding: "5",
                // make attribute a String to avoid IntoAttributeValue ambiguity
                background: (if msg.role == "user" {
                  "rgb(200,230,255)"
                } else if msg.role == "assistant" {
                  "rgb(230,230,230)"
                } else {
                  "rgb(240,220,255)"
                }).to_string(),
                label { "{msg.role}:" }   // this style works for text nodes
                text  { "{msg.content}" }
              }
            } // end for
          } // ScrollView
        } 

      // Input box (bottom 25%)
      rect {
        height: "25%",
        width: "100%",
        padding: "5",
        background: "rgb(250,250,250)",
        corner_radius: "6",
        border: "0.3 inner black",
        label { "Your message:" },
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

pub fn ai_input_editor_view(state: &mut AppState) -> Element {
  let mut focus_text = state.focus_text;
  let mut editable_nl = state.editable_nl;
  rsx!(
    rect {
      width: "20%",
      height: "100%",
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
