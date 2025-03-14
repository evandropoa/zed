use gpui::AnyElement;
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct SettingsContainer {
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for SettingsContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsContainer {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for SettingsContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SettingsContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex().px_2().gap_1().children(self.children)
    }
}
