use freya::prelude::*;
pub struct TableData {
  pub headers: Vec<String>,
  pub rows: Vec<Vec<String>>,
}

pub fn results_table(results: &Signal<TableData>) -> Element {
  rsx!(
    rect {
      width: "100%",
      height: "50%",
      padding: "5",
      spacing: "5",
      Table {
        columns: results.read().headers.len().max(1),
        TableHead {
          TableRow {
            for (i, col) in results.read().headers.iter().enumerate() {
              TableCell {
                key: "{i}",
                label { text_align: "left", font_size: "16", font_weight: "bold", "{col}" }
              }
            }
          }
        }
        TableBody {
          for (idx, row) in results.read().rows.iter().enumerate() {
            TableRow {
              key: "{idx}",
              for (col_idx, cell) in row.iter().enumerate() {
                TableCell {
                  key: "{idx}-{col_idx}",
                  label { text_align: "left", font_size: "12", "{cell}" }
                }
              }
            }
          }
        }
      }
    }
  )
}
