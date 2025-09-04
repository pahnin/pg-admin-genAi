use freya::prelude::*;

pub fn modal(mut show_modal: Signal<bool>, tables: Signal<Vec<String>>) -> Element {
  rsx! {
      if show_modal() {
        rect {
          width: "100%",
          height: "100%",
          position: "absolute",
          layer: "-100",
          rect {
            background: "rgb(0,0,0)",
            opacity: "0.5",
            width: "100%",
            height: "100%",
            position: "absolute",
            layer: "-101",
            onclick: move |_| show_modal.set(false),
          }
          rect {
            width: "100%",
            height: "100%",
            position: "absolute",
            layer: "-150",
            main_align: "center",
            cross_align: "center",

            rect {
              width: "600",
              height: "400",
              rect {
                width: "100%",
                padding: "12",
                opacity: "1",
                background: "white",
                corner_radius: "8",
                direction: "vertical",
                spacing: "8",
                label { "Tables: {tables.read().len()}" }
                ScrollView {
                  for table in tables.read().iter() {
                    // For future: make this expandable accordion
                    label { "{table}" }
                  }
                }
              }
            }
          }
        }
      }
  }
}
