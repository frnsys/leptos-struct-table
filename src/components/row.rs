use crate::table_row::TableRow;
use crate::{ChangeEvent, EventHandler};
use leptos::*;

/// The default table row renderer. Uses the `<tr>` element. Please note that this
/// is **NOT** a `#[component]`.
#[allow(unused_variables)]
pub fn DefaultTableRowRenderer<Row>(
    // The class attribute for the row element. Generated by the classes provider.
    class: Signal<String>,
    // The row to render.
    row: Row,
    // The index of the row. Starts at 0 for the first body row.
    index: usize,
    // The selected state of the row. True, when the row is selected.
    selected: Signal<bool>,
    // Event handler callback when this row is selected
    on_select: EventHandler<web_sys::MouseEvent>,
    // Event handler callback for changes
    on_change: EventHandler<ChangeEvent<Row>>,
) -> impl IntoView
where
    Row: TableRow + Clone + 'static,
{
    view! {
        <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
            {row.render_row(index, on_change)}
        </tr>
    }
}

/// The default row placeholder renderer which is just a div that is set to the
/// appropriate height. This is used in place of rows that are not shown
/// before and after the currently visible rows.
pub fn DefaultRowPlaceholderRenderer(height: Signal<f64>) -> impl IntoView {
    view! { <div style:height=move || format!("{}px", height.get())></div> }
}

/// The default error row renderer which just displays the error message when
/// a row fails to load, i.e. when [`TableDataProvider::get_rows`] returns an `Err(..)`.
#[allow(unused_variables)]
pub fn DefaultErrorRowRenderer(err: String, index: usize, col_count: usize) -> impl IntoView {
    view! { <tr><td colspan=col_count>{err}</td></tr> }
}

/// The default loading row renderer which just displays a loading indicator.
#[allow(unused_variables)]
pub fn DefaultLoadingRowRenderer(
    class: Signal<String>,
    cell_class: Signal<String>,
    inner_cell_class: Signal<String>,
    index: usize,
    col_count: usize,
) -> impl IntoView {
    view! {
        <tr class=class>
            {
                (0..col_count).map(|_| view! {
                    <td class=cell_class>
                        <div class=inner_cell_class></div>
                        " "
                    </td>
                }).collect_view()
            }
        </tr>
    }
}
