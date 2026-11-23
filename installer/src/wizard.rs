use crate::Args;
use panopticon_setup::{
    platform::{self, wide},
    InstallOptions, PRODUCT,
};
use std::{
    cell::RefCell,
    mem::zeroed,
    path::PathBuf,
    ptr::{null, null_mut},
    sync::{
        atomic::AtomicBool,
        mpsc::{channel, Receiver, Sender},
    },
};
use windows_sys::Win32::{
    Foundation::*,
    Globalization::GetUserDefaultUILanguage,
    Graphics::Gdi::*,
    Storage::FileSystem::GetDiskFreeSpaceExW,
    System::LibraryLoader::{FindResourceW, GetModuleHandleW, LoadResource, LockResource},
    UI::{
        Controls::*,
        Input::KeyboardAndMouse::{EnableWindow, SetFocus},
        WindowsAndMessaging::*,
    },
};

include!(concat!(env!("OUT_DIR"), "/wizard_languages.rs"));

const DIRECTORY_PAGE: u16 = 103;
const MAIN_DIALOG: u16 = 105;
const PROGRESS_PAGE: u16 = 106;
const ILLUSTRATED_PAGE: u16 = 201;
const HEADER_CONTROLS: &[i32] = &[1034, 1035, 1036, 1037, 1038, 1039, 1256, 1028];
const ENGINE_EVENT: u32 = WM_APP + 7;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Directory,
    Welcome,
    Progress,
    Finish,
}
enum Event {
    Progress(String),
    Done(Result<(), String>),
}

struct Wizard {
    args: Args,
    directory: PathBuf,
    strings: &'static [&'static str],
    page: Page,
    child: HWND,
    title_font: HFONT,
    finish_font: HFONT,
    sidebar: HBITMAP,
    required_bytes: u64,
    step: usize,
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

fn language(uninstall: bool, locale: u16) -> &'static [&'static str] {
    let mut tables = TABLES.iter().filter(|(un, _, _)| *un == uninstall);
    tables
        .clone()
        .find(|(_, id, _)| *id == locale)
        .or_else(|| {
            tables
                .clone()
                .find(|(_, id, _)| id & 0x3ff == locale & 0x3ff)
        })
        .or_else(|| tables.find(|(_, id, _)| *id == 1033))
        .unwrap()
        .2
}

unsafe fn template_ptr(id: u16) -> *const DLGTEMPLATE {
    let module = GetModuleHandleW(null());
    let kind = if id == ILLUSTRATED_PAGE {
        RT_DIALOG
    } else {
        10usize as _
    };
    let info = FindResourceW(module, id as usize as _, kind);
    LockResource(LoadResource(module, info)).cast()
}

unsafe fn caption(parent: HWND, id: i32, text: &str) {
    SetDlgItemTextW(parent, id, wide(text).as_ptr());
}

unsafe fn edit_path(child: HWND) -> PathBuf {
    use std::os::windows::ffi::OsStringExt;
    let ctrl = GetDlgItem(child, 1019);
    let mut buffer = vec![0; (GetWindowTextLengthW(ctrl) + 1) as usize];
    let count = GetWindowTextW(ctrl, buffer.as_mut_ptr(), buffer.len() as i32);
    PathBuf::from(std::ffi::OsString::from_wide(&buffer[..count as usize]))
}

unsafe fn visible(hwnd: HWND, id: i32, show: bool) {
    ShowWindow(GetDlgItem(hwnd, id), if show { SW_SHOW } else { SW_HIDE });
}

unsafe fn bounds_in(hwnd: HWND, control: HWND) -> RECT {
    let mut rect: RECT = zeroed();
    GetWindowRect(control, &mut rect);
    MapWindowPoints(null_mut(), hwnd, (&mut rect as *mut RECT).cast(), 2);
    rect
}

unsafe fn bold_font(hwnd: HWND, points: i32) -> HFONT {
    let dc = GetDC(hwnd);
    let height = -(points * GetDeviceCaps(dc, LOGPIXELSY as i32) + 36) / 72;
    ReleaseDC(hwnd, dc);
    CreateFontW(
        height,
        0,
        0,
        0,
        700,
        0,
        0,
        0,
        DEFAULT_CHARSET as u32,
        0,
        0,
        0,
        0,
        wide("MS Shell Dlg").as_ptr(),
    )
}

unsafe fn white(dc: HDC) -> isize {
    SetTextColor(dc, 0);
    SetBkColor(dc, 0xffffff);
    GetStockObject(WHITE_BRUSH) as isize
}

// Child dialogs keep their resource-defined tab order. The outer dialog owns navigation.
unsafe extern "system" fn page_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> isize {
    match msg {
        WM_INITDIALOG => {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, l);
            1
        }
        WM_COMMAND => {
            PostMessageW(GetParent(hwnd), WM_COMMAND, w, l);
            1
        }
        WM_CTLCOLORDLG | WM_CTLCOLORSTATIC | WM_CTLCOLORBTN
            if GetWindowLongPtrW(hwnd, GWLP_USERDATA) == ILLUSTRATED_PAGE as isize =>
        {
            white(w as HDC)
        }
        _ => 0,
    }
}

impl Wizard {
    fn label(&self, install: usize, uninstall: usize) -> &'static str {
        self.strings[if self.args.uninstall {
            uninstall
        } else {
            install
        }]
    }

    unsafe fn init(&mut self, hwnd: HWND) {
        SetWindowTextW(hwnd, wide(self.strings[1]).as_ptr());
        self.title_font = bold_font(hwnd, self.label(54, 53).parse().unwrap_or(8));
        self.finish_font = bold_font(hwnd, 12);
        SendDlgItemMessageW(hwnd, 1037, WM_SETFONT, self.title_font as usize, 0);
        let module = GetModuleHandleW(null());
        for (kind, size) in [
            (ICON_SMALL, GetSystemMetrics(SM_CXSMICON)),
            (ICON_BIG, GetSystemMetrics(SM_CXICON)),
        ] {
            let icon = LoadImageW(module, 103usize as _, IMAGE_ICON, size, size, LR_SHARED);
            SendMessageW(hwnd, WM_SETICON, kind as usize, icon as isize);
        }
        let brand = format!("{} ", self.strings[0]);
        // The MUI /BRANDING footer stops the etched separator where the brand ends.
        let brand_wide = wide(&brand);
        let dc = GetDC(hwnd);
        let old = SelectObject(dc, SendMessageW(hwnd, WM_GETFONT, 0, 0) as HGDIOBJ);
        let mut extent: SIZE = zeroed();
        GetTextExtentPoint32W(
            dc,
            brand_wide.as_ptr(),
            (brand_wide.len() - 1) as i32,
            &mut extent,
        );
        SelectObject(dc, old);
        ReleaseDC(hwnd, dc);
        for id in [1256, 1028] {
            let control = GetDlgItem(hwnd, id);
            let rect = bounds_in(hwnd, control);
            SetWindowPos(
                control,
                null_mut(),
                0,
                0,
                extent.cx + 2,
                rect.bottom - rect.top,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
            );
            caption(hwnd, id, &brand);
        }
        let separator = GetDlgItem(hwnd, 1035);
        let rect = bounds_in(hwnd, separator);
        let x = rect.left + extent.cx + 2;
        SetWindowPos(
            separator,
            null_mut(),
            x,
            rect.top,
            rect.right - x,
            rect.bottom - rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
        caption(hwnd, 2, self.label(62, 60));
        visible(hwnd, 3, false);
        self.show(hwnd, self.page);
    }

    unsafe fn show(&mut self, hwnd: HWND, page: Page) {
        self.page = page;
        if !self.child.is_null() {
            DestroyWindow(self.child);
            self.child = null_mut();
        }
        if !self.sidebar.is_null() {
            DeleteObject(self.sidebar);
            self.sidebar = null_mut();
        }
        let full = matches!(page, Page::Welcome | Page::Finish);
        for &id in HEADER_CONTROLS {
            visible(hwnd, id, !full);
        }
        visible(hwnd, 1045, full);
        let template = match page {
            Page::Directory => DIRECTORY_PAGE,
            Page::Progress => PROGRESS_PAGE,
            Page::Welcome | Page::Finish => ILLUSTRATED_PAGE,
        };
        let position = bounds_in(hwnd, GetDlgItem(hwnd, if full { 1044 } else { 1018 }));
        let module = GetModuleHandleW(null());
        self.child = CreateDialogIndirectParamW(
            module,
            template_ptr(template),
            hwnd,
            Some(page_proc),
            template as isize,
        );
        if self.child.is_null() {
            EndDialog(hwnd, -1);
            return;
        }
        SetWindowPos(
            self.child,
            HWND_TOP,
            position.left,
            position.top,
            position.right - position.left,
            position.bottom - position.top,
            SWP_NOACTIVATE,
        );
        match page {
            Page::Directory => {
                caption(hwnd, 1037, self.strings[37]);
                caption(hwnd, 1038, self.strings[38]);
                for (id, string) in [(1006, 63), (1020, 64), (1001, 65)] {
                    caption(self.child, id, self.strings[string]);
                }
                SetDlgItemTextW(self.child, 1019, wide(&self.directory).as_ptr());
                self.disk_space(hwnd);
            }
            Page::Progress => {
                caption(hwnd, 1037, self.label(39, 41));
                caption(hwnd, 1038, self.label(40, 42));
                visible(self.child, 1027, false);
                self.step = 0;
                SendDlgItemMessageW(
                    self.child,
                    1004,
                    PBM_SETRANGE32,
                    0,
                    if self.args.uninstall { 2 } else { 5 },
                );
            }
            Page::Welcome | Page::Finish => {
                let welcome = page == Page::Welcome;
                caption(
                    self.child,
                    2001,
                    if welcome {
                        self.strings[38]
                    } else {
                        self.label(47, 48)
                    },
                );
                caption(
                    self.child,
                    2002,
                    if welcome {
                        self.strings[40]
                    } else {
                        self.strings[52]
                    },
                );
                SendDlgItemMessageW(self.child, 2001, WM_SETFONT, self.finish_font as usize, 0);
                let run = page == Page::Finish && !self.args.uninstall;
                visible(self.child, 2003, run);
                if run {
                    // The original install finish uses a 40-DLU body above the run checkbox.
                    let rect = bounds_in(self.child, GetDlgItem(self.child, 2002));
                    let mut body: RECT = zeroed();
                    body.bottom = 40;
                    MapDialogRect(self.child, &mut body);
                    SetWindowPos(
                        GetDlgItem(self.child, 2002),
                        null_mut(),
                        0,
                        0,
                        rect.right - rect.left,
                        body.bottom,
                        SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                    caption(self.child, 2003, self.strings[53]);
                    SendDlgItemMessageW(self.child, 2003, BM_SETCHECK, 1, 0);
                }
                let picture = GetDlgItem(self.child, 2000);
                let mut rect: RECT = zeroed();
                GetClientRect(picture, &mut rect);
                self.sidebar = LoadImageW(
                    module,
                    401usize as _,
                    IMAGE_BITMAP,
                    rect.right,
                    rect.bottom,
                    0,
                );
                SendMessageW(
                    picture,
                    STM_SETIMAGE,
                    IMAGE_BITMAP as usize,
                    self.sidebar as isize,
                );
            }
        }
        caption(
            hwnd,
            1,
            if page == Page::Finish {
                self.label(45, 47)
            } else {
                self.label(68, 62)
            },
        );
        EnableWindow(GetDlgItem(hwnd, 1), (page != Page::Progress) as i32);
        EnableWindow(
            GetDlgItem(hwnd, 2),
            matches!(page, Page::Directory | Page::Welcome) as i32,
        );
        SendMessageW(hwnd, DM_SETDEFID, 1, 0);
        ShowWindow(self.child, SW_SHOW);
        let focus = if page == Page::Directory {
            GetDlgItem(self.child, 1019)
        } else if page == Page::Finish && !self.args.uninstall {
            GetDlgItem(self.child, 2003)
        } else {
            GetDlgItem(hwnd, 1)
        };
        SetFocus(focus);
        if page == Page::Directory {
            SendMessageW(focus, EM_SETSEL, 0, -1);
        }
        InvalidateRect(hwnd, null(), 1);
    }

    fn size_text(&self, bytes: u64) -> String {
        let mut unit = 0;
        let mut divisor = 1u64;
        while unit < 3 && bytes / divisor >= 1024 {
            unit += 1;
            divisor *= 1024;
        }
        if unit == 0 {
            return format!("{bytes} {}", self.strings[32]);
        }
        let tenths = (bytes % divisor) * 10 / divisor;
        format!(
            "{}.{tenths}{}{}",
            bytes / divisor,
            self.strings[32 + unit],
            self.strings[32]
        )
    }

    unsafe fn disk_space(&self, hwnd: HWND) {
        let path = edit_path(self.child);
        caption(
            self.child,
            1023,
            &format!("{}{}", self.strings[4], self.size_text(self.required_bytes)),
        );
        let mut available = 0;
        let disk = path.ancestors().find(|p| p.is_dir());
        let readable = disk.is_some_and(|p| {
            GetDiskFreeSpaceExW(wide(p).as_ptr(), &mut available, null_mut(), null_mut()) != 0
        });
        let value = if readable {
            format!("{}{}", self.strings[3], self.size_text(available))
        } else {
            String::new()
        };
        caption(self.child, 1024, &value);
        EnableWindow(
            GetDlgItem(hwnd, 1),
            (path.is_absolute() && (!readable || available >= self.required_bytes)) as i32,
        );
    }

    unsafe fn next(&mut self, hwnd: HWND) {
        if self.page == Page::Progress {
            return;
        }
        if self.page == Page::Finish {
            if !self.args.uninstall && SendDlgItemMessageW(self.child, 2003, BM_GETCHECK, 0, 0) == 1
            {
                let _ = std::process::Command::new("explorer.exe")
                    .arg(self.directory.join(panopticon_setup::HOST))
                    .spawn();
            }
            EndDialog(hwnd, 1);
            return;
        }
        if self.page == Page::Directory {
            self.directory = edit_path(self.child);
            if let Err(e) = panopticon_setup::validate_directory(&self.directory) {
                MessageBoxW(
                    hwnd,
                    wide(e).as_ptr(),
                    wide(self.strings[1]).as_ptr(),
                    MB_ICONWARNING,
                );
                return;
            }
        }
        self.run_operation(hwnd);
    }

    unsafe fn run_operation(&mut self, hwnd: HWND) {
        self.show(hwnd, Page::Progress);
        if self.child.is_null() {
            return;
        }
        let args = self.args.clone();
        let dir = self.directory.clone();
        let tx = self.tx.clone();
        let window = hwnd as usize;
        std::thread::spawn(move || {
            let notify = |event| {
                let _ = tx.send(event);
                PostMessageW(window as HWND, ENGINE_EVENT, 0, 0);
            };
            let progress = |s| notify(Event::Progress(s));
            let result = if args.uninstall {
                panopticon_setup::uninstall(&dir, progress).map(|_| ())
            } else {
                let options = InstallOptions {
                    directory: dir,
                    scope: args.scope,
                    desktop_shortcut: args.desktop,
                    start_menu_shortcut: args.start_menu,
                };
                panopticon_setup::install(&options, &AtomicBool::new(false), progress).map(|_| ())
            };
            notify(Event::Done(result));
        });
    }

    unsafe fn events(&mut self, hwnd: HWND) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::Progress(s) => {
                    caption(self.child, 1006, &s);
                    self.step += 1;
                    SendDlgItemMessageW(self.child, 1004, PBM_SETPOS, self.step, 0);
                }
                Event::Done(Ok(())) => self.show(hwnd, Page::Finish),
                Event::Done(Err(e)) => {
                    caption(hwnd, 1037, self.label(43, 45));
                    let before_removal = self.args.uninstall && self.step == 0;
                    caption(hwnd, 1038, if before_removal { "" } else { self.label(44, 46) });
                    let icon = if before_removal { MB_ICONWARNING } else { MB_ICONERROR };
                    if MessageBoxW(
                        hwnd,
                        wide(&e).as_ptr(),
                        wide(self.strings[1]).as_ptr(),
                        icon | MB_RETRYCANCEL,
                    ) == IDRETRY
                    {
                        if self.args.uninstall {
                            self.run_operation(hwnd);
                        } else {
                            self.show(hwnd, Page::Directory);
                        }
                    } else {
                        EndDialog(hwnd, 2);
                    }
                }
            }
        }
    }
}

unsafe extern "system" fn dialog_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> isize {
    if msg == WM_INITDIALOG {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, l);
    }
    if msg == WM_CTLCOLORSTATIC {
        let id = GetDlgCtrlID(l as HWND);
        if matches!(id, 1034 | 1037 | 1038 | 1039) {
            return white(w as HDC);
        }
        if matches!(id, 1256 | 1028) {
            SetTextColor(w as HDC, GetSysColor(COLOR_GRAYTEXT));
            SetBkColor(w as HDC, GetSysColor(COLOR_BTNFACE));
            return GetSysColorBrush(COLOR_BTNFACE) as isize;
        }
    }
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const RefCell<Wizard>;
    if ptr.is_null() {
        return 0;
    }
    let Ok(mut app) = (*ptr).try_borrow_mut() else {
        return 0;
    };
    match msg {
        WM_INITDIALOG => {
            app.init(hwnd);
            0
        }
        WM_CLOSE => {
            if app.page != Page::Progress {
                EndDialog(hwnd, 0);
            }
            1
        }
        WM_COMMAND => {
            match (w & 0xffff) as i32 {
                1 => app.next(hwnd),
                2 => {
                    if app.page != Page::Progress {
                        EndDialog(hwnd, 0);
                    }
                }
                1001 if app.page == Page::Directory => {
                    let dir = edit_path(app.child);
                    if let Ok(Some(path)) =
                        platform::choose_directory(hwnd as usize, app.strings[66], &dir)
                    {
                        let selected = if path
                            .file_name()
                            .is_some_and(|n| n.eq_ignore_ascii_case(PRODUCT))
                        {
                            path
                        } else {
                            path.join(PRODUCT)
                        };
                        SetDlgItemTextW(app.child, 1019, wide(selected).as_ptr());
                    }
                }
                1019 if app.page == Page::Directory && w >> 16 == EN_CHANGE as usize => {
                    app.disk_space(hwnd)
                }
                _ => return 0,
            }
            1
        }
        ENGINE_EVENT => {
            app.events(hwnd);
            1
        }
        _ => 0,
    }
}

pub fn run(args: Args, directory: PathBuf) -> Result<(), String> {
    let (tx, rx) = channel();
    let size = std::env::current_exe()
        .and_then(std::fs::metadata)
        .map_err(|e| e.to_string())?
        .len();
    unsafe {
        let mut app = RefCell::new(Wizard {
            page: if args.uninstall {
                Page::Welcome
            } else {
                Page::Directory
            },
            strings: language(args.uninstall, GetUserDefaultUILanguage()),
            args,
            directory,
            tx,
            rx,
            child: null_mut(),
            title_font: null_mut(),
            finish_font: null_mut(),
            sidebar: null_mut(),
            step: 0,
            required_bytes: panopticon_setup::PAYLOAD.len() as u64 + size + 4096,
        });
        InitCommonControls();
        let ret = DialogBoxIndirectParamW(
            GetModuleHandleW(null()),
            template_ptr(MAIN_DIALOG),
            null_mut(),
            Some(dialog_proc),
            &mut app as *mut _ as isize,
        );
        let last_error = std::io::Error::last_os_error();
        let state = app.into_inner();
        for resource in [state.title_font, state.finish_font, state.sidebar] {
            if !resource.is_null() {
                DeleteObject(resource);
            }
        }
        if ret == -1 {
            return Err(last_error.to_string());
        }
    }
    Ok(())
}
