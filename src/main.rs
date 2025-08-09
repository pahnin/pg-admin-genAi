use freya::prelude::*;

fn main() {
  // launch_with_props is gettting renamed to launch_with_params TODO
  launch_with_props(app, "Pg Admin Console", (900.0, 650.0));
}

fn app() -> Element {
  let mut editable = use_editable(
    || {
      EditableConfig::new(
        "Type you SQL here"
        .trim()
        .to_string(),
      )
        .with_allow_tabs(true)
    },
    EditableMode::MultipleLinesSingleEditor,
  );

  let cursor_reference = editable.cursor_attr();
  let highlights = editable.highlights_attr(0);
  let editor = editable.editor().read();
  let cursor_char = editor.cursor_pos();

  let onmousedown = move |e: MouseEvent| {
    editable.process_event(&EditableEvent::MouseDown(e.data, 0));
  };

  let onmousemove = move |e: MouseEvent| {
    editable.process_event(&EditableEvent::MouseMove(e.data, 0));
  };

  let onclick = move |_: MouseEvent| {
    editable.process_event(&EditableEvent::Click);
  };

  let onglobalkeydown = move |e: KeyboardEvent| {
    editable.process_event(&EditableEvent::KeyDown(e.data));
  };

  let onglobalkeyup = move |e: KeyboardEvent| {
    editable.process_event(&EditableEvent::KeyUp(e.data));
  };
  let i = 0;

  rsx!(
    Body {
      padding: "10",
      spacing: "10",
      rect {
        width: "100%",
        height: "30%",
        padding: "5",
        spacing: "5",
        ScrollView {
          width: "100%",
          height: "calc(100% - 30)",
          scroll_with_arrows: false,
          paragraph {
            width: "100%",
            height: "100%",
            main_align: "center",
            cursor_id: "0",
            cursor_index: "{cursor_char}",
            cursor_mode: "editable",
            cursor_color: "black",
            highlights,
            cursor_reference,
            onclick,
            onmousemove,
            onmousedown,
            onglobalkeydown,
            onglobalkeyup,
            text {
              "{editable.editor()}"
            }
          }
        }
        label {
          color: "black",
          height: "30",
          "{editor.cursor_row()}:{editor.cursor_col()}"
        }
      }
      rect {
        width: "100%",
        height: "70%",
        padding: "5",
        spacing: "5",



        Table {
          columns: 1,
          TableHead { 
            TableRow { 
              TableCell {
                key: "{i}",
                label { width: "100%", text_align: "center", "Results" } 
              } 
            } 
          }
          TableBody {
            TableRow {
              key: "{i}",
              TableCell { 
                key: "{i}",
                label { 
                  width: "100%", 
                  text_align: "center", 
                  "Execute SQL to see results" 
                } 
              }
            }
          }
        }
      }
    }
  )
}

