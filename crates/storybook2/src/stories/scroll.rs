use gpui2::{
    div, px, Component, ParentElement, SharedString, Styled, View, VisualContext, WindowContext,
};
use theme2::theme;

pub struct ScrollStory {
    text: View<()>,
}

impl ScrollStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        cx.build_view(|cx| (), move |_, cx| checkerboard(cx, 1))
    }
}

fn checkerboard<S>(cx: &mut WindowContext, depth: usize) -> impl Component<S>
where
    S: 'static + Send + Sync,
{
    let theme = theme(cx);
    let color_1 = theme.git_created;
    let color_2 = theme.git_modified;

    div()
        .id("parent")
        .bg(theme.background)
        .size_full()
        .overflow_scroll()
        .children((0..10).map(|row| {
            div()
                .w(px(1000.))
                .h(px(100.))
                .flex()
                .flex_row()
                .children((0..10).map(|column| {
                    let id = SharedString::from(format!("{}, {}", row, column));
                    let bg = if row % 2 == column % 2 {
                        color_1
                    } else {
                        color_2
                    };
                    div().id(id).bg(bg).size(px(100. / depth as f32)).when(
                        row >= 5 && column >= 5,
                        |d| {
                            d.overflow_scroll()
                                .child(div().size(px(50.)).bg(color_1))
                                .child(div().size(px(50.)).bg(color_2))
                                .child(div().size(px(50.)).bg(color_1))
                                .child(div().size(px(50.)).bg(color_2))
                        },
                    )
                }))
        }))
}
