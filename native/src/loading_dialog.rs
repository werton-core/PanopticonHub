use super::{video::EngineObject, Interface};
use std::{
    cell::OnceCell,
    ffi::{c_char, c_int, CStr},
};

type Panel = usize; // vgui::VPANEL (uintp)
type GetPanel = unsafe extern "system" fn(Interface, c_int) -> Panel;
type ChildCount = unsafe extern "system" fn(Interface, Panel) -> c_int;
type GetChild = unsafe extern "system" fn(Interface, Panel, c_int) -> Panel;
type GetName = unsafe extern "system" fn(Interface, Panel) -> *const c_char;
type IsVisible = unsafe extern "system" fn(Interface, Panel) -> bool;
type SetVisible = unsafe extern "system" fn(Interface, Panel, bool);

// Windows x64 layouts of Valve's IEngineVGui and IPanel, including the destructor.
const GAME_UI: c_int = 1; // PANEL_GAMEUIDLL
const GET_PANEL: usize = 1;
const SET_VISIBLE: usize = 14;
const IS_VISIBLE: usize = 15;
const GET_CHILD_COUNT: usize = 17;
const GET_CHILD: usize = 18;
const GET_NAME: usize = 36;

struct GameUi {
    engine: EngineObject,
    panels: EngineObject,
    get_panel: GetPanel,
    child_count: ChildCount,
    get_child: GetChild,
    get_name: GetName,
    is_visible: IsVisible,
    set_visible: SetVisible,
}

impl GameUi {
    fn load() -> Option<Self> {
        let engine = EngineObject::get("engine.dll", c"VEngineVGui001")?;
        let panels = EngineObject::get("vgui2.dll", c"VGUI_Panel009")?;
        Some(Self {
            get_panel: unsafe { std::mem::transmute::<_, GetPanel>(engine.method(GET_PANEL)?) },
            child_count: unsafe {
                std::mem::transmute::<_, ChildCount>(panels.method(GET_CHILD_COUNT)?)
            },
            get_child: unsafe { std::mem::transmute::<_, GetChild>(panels.method(GET_CHILD)?) },
            get_name: unsafe { std::mem::transmute::<_, GetName>(panels.method(GET_NAME)?) },
            is_visible: unsafe { std::mem::transmute::<_, IsVisible>(panels.method(IS_VISIBLE)?) },
            set_visible: unsafe {
                std::mem::transmute::<_, SetVisible>(panels.method(SET_VISIBLE)?)
            },
            engine,
            panels,
        })
    }

    fn named(&self, panel: Panel, name: &CStr) -> bool {
        let actual = unsafe { (self.get_name)(self.panels.interface, panel) };
        !actual.is_null()
            && unsafe { CStr::from_ptr(actual) }
                .to_bytes()
                .eq_ignore_ascii_case(name.to_bytes())
    }

    fn children(&self, panel: Panel) -> impl Iterator<Item = Panel> + '_ {
        let count = unsafe { (self.child_count)(self.panels.interface, panel) };
        (0..count.clamp(0, 512))
            .map(move |index| unsafe { (self.get_child)(self.panels.interface, panel, index) })
            .filter(|&child| child != 0)
    }

    fn visible(&self, panel: Panel) -> bool {
        unsafe { (self.is_visible)(self.panels.interface, panel) }
    }

    fn is_progress_dialog(&self, panel: Panel) -> bool {
        if !self.named(panel, c"LoadingDialog") {
            return false;
        }
        let mut progress = false;
        let mut cancel = false;
        for child in self.children(panel) {
            // The same frame name is used by the disconnect dialog.
            if self.named(child, c"CopyButton") && self.visible(child) {
                return false;
            }
            progress |= self.named(child, c"Progress") && self.visible(child);
            cancel |= self.named(child, c"CancelButton");
        }
        progress && cancel
    }

    fn find(&self, panel: Panel, depth: u8) -> Option<Panel> {
        let mut children = self.children(panel);
        children
            .find(|&child| self.is_progress_dialog(child))
            .or_else(|| {
                if depth == 0 {
                    return None;
                }
                self.children(panel)
                    .filter(|&child| self.visible(child))
                    .find_map(|child| self.find(child, depth - 1))
            })
    }

    fn hide(&self) -> bool {
        let root = unsafe { (self.get_panel)(self.engine.interface, GAME_UI) };
        let Some(dialog) = (root != 0).then(|| self.find(root, 2)).flatten() else {
            return false;
        };
        if self.visible(dialog) {
            unsafe { (self.set_visible)(self.panels.interface, dialog, false) };
        }
        !self.visible(dialog)
    }
}

thread_local! {
    static GAME_UI_API: OnceCell<Option<GameUi>> = const { OnceCell::new() };
}

pub(super) fn hide() -> bool {
    GAME_UI_API.with(|api| {
        api.get_or_init(GameUi::load)
            .as_ref()
            .is_some_and(GameUi::hide)
    })
}
