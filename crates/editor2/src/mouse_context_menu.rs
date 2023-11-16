use crate::{
    element::PointForPosition, DisplayPoint, Editor, EditorMode, FindAllReferences, GoToDefinition,
    GoToTypeDefinition, Rename, RevealInFinder, SelectMode, ToggleCodeActions,
};
use gpui::{Pixels, Point, ViewContext};
use ui2::{ContextMenu, ContextMenuItem, Label};

pub fn deploy_context_menu(
    editor: &mut Editor,
    position: Point<Pixels>,
    point: PointForPosition,
    cx: &mut ViewContext<Editor>,
) {
    // if !editor.focused {
    //     cx.focus_self();
    // }

    // // Don't show context menu for inline editors
    // if editor.mode() != EditorMode::Full {
    //     return;
    // }

    // Don't show the context menu if there isn't a project associated with this editor
    if editor.project.is_none() {
        return;
    }
    let p = point.previous_valid;
    // Move the cursor to the clicked location so that dispatched actions make sense
    editor.change_selections(None, cx, |s| {
        s.clear_disjoint();
        s.set_pending_display_range(p..p, SelectMode::Character);
    });

    *editor.context_menu.write() = Some(crate::ContextMenu::MouseContextMenu(
        crate::MouseContextMenu {
            menu: ContextMenu::new([
                ContextMenuItem::entry(Label::new("Rename Symbol"), Rename),
                ContextMenuItem::entry(Label::new("Go to Definition"), GoToDefinition),
                ContextMenuItem::entry(Label::new("Go to Type Definition"), GoToTypeDefinition),
                ContextMenuItem::entry(Label::new("Find All References"), FindAllReferences),
                ContextMenuItem::entry(
                    Label::new("Code Actions"),
                    ToggleCodeActions {
                        deployed_from_indicator: false,
                    },
                ),
                ContextMenuItem::separator(),
                ContextMenuItem::entry(Label::new("Reveal in Finder"), RevealInFinder),
            ]),
            visible: true,
            exact_point: point.exact_unclipped,
        },
    ));
    cx.notify();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
//     use indoc::indoc;

//     #[gpui::test]
//     async fn test_mouse_context_menu(cx: &mut gpui::TestAppContext) {
//         init_test(cx, |_| {});

//         let mut cx = EditorLspTestContext::new_rust(
//             lsp::ServerCapabilities {
//                 hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
//                 ..Default::default()
//             },
//             cx,
//         )
//         .await;

//         cx.set_state(indoc! {"
//             fn teˇst() {
//                 do_work();
//             }
//         "});
//         let point = cx.display_point(indoc! {"
//             fn test() {
//                 do_wˇork();
//             }
//         "});
//         cx.update_editor(|editor, cx| deploy_context_menu(editor, Default::default(), point, cx));

//         cx.assert_editor_state(indoc! {"
//             fn test() {
//                 do_wˇork();
//             }
//         "});
//         cx.editor(|editor, app| assert!(editor.mouse_context_menu.read(app).visible()));
//     }
// }
