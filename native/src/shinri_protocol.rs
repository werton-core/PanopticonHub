//! Подтверждённые проверки протокола Shinri; меняется только загруженный образ.
use std::{ffi::c_void, sync::Mutex};
use windows_sys::{
    w,
    Win32::{
        Foundation::{
            CloseHandle, GetLastError, ERROR_INVALID_PARAMETER, ERROR_NO_MORE_FILES, HANDLE,
            INVALID_HANDLE_VALUE, WAIT_OBJECT_0,
        },
        System::{
            Diagnostics::{
                Debug::{
                    FlushInstructionCache, GetThreadContext, ReadProcessMemory, CONTEXT,
                    CONTEXT_CONTROL_AMD64,
                },
                ToolHelp::{
                    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD,
                    THREADENTRY32,
                },
            },
            LibraryLoader::{GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_PIN},
            Memory::{
                VirtualProtect, VirtualQuery, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE,
            },
            Threading::{
                GetCurrentProcess, GetCurrentProcessId, GetCurrentThreadId, OpenThread,
                ResumeThread, SuspendThread, WaitForSingleObject, THREAD_GET_CONTEXT,
                THREAD_QUERY_INFORMATION, THREAD_SUSPEND_RESUME,
            },
        },
    },
};

const CONNECT: &[u8] = b"\xba\x6b\0\0\0\x48\x8d\x4c\x24\0\xe8\0\0\0\0\xba\x18\0\0\0";
const HOST: &[u8] = b"\xc7\x47\x20\x18\0\0\0\x48\x8b\xcb\x48\x8b\x03\xff\x50\x70";
const CHECK: &[u8] = b"\x83\x7e\x20\x18\x74\0\x48\x8b\x0d\0\0\0\0\x48\x8b\x01\xff\x50\x30\x84\xc0\x74\0\x83\x7e\x20\x0c\x7d";

#[derive(Clone, Debug)]
struct Patch {
    offset: usize,
    before: Vec<u8>,
    after: Vec<u8>,
}
struct Active {
    base: usize,
    patches: Vec<Patch>,
    enabled: bool,
    uncertain: bool,
}
static ACTIVE: Mutex<Option<Active>> = Mutex::new(None);
#[cfg(test)]
static FAIL_WRITE: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(-1);

fn u32_at(image: &[u8], at: usize) -> Result<u32, String> {
    image
        .get(at..at + 4)
        .and_then(|x| x.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| "Повреждён заголовок engine.dll.".into())
}
fn section(image: &[u8], wanted: &[u8]) -> Result<std::ops::Range<usize>, String> {
    let pe = u32_at(image, 0x3c)? as usize;
    if image.get(..2) != Some(b"MZ") || image.get(pe..pe + 4) != Some(b"PE\0\0") {
        return Err("Неизвестный формат engine.dll.".into());
    }
    let count = image
        .get(pe + 6..pe + 8)
        .map(|v| u16::from_le_bytes(v.try_into().unwrap()) as usize)
        .ok_or("Нет секций engine.dll.")?;
    let size = image
        .get(pe + 20..pe + 22)
        .map(|v| u16::from_le_bytes(v.try_into().unwrap()) as usize)
        .ok_or("Нет заголовка engine.dll.")?;
    if count > 96 {
        return Err("Некорректные секции engine.dll.".into());
    }
    for i in 0..count {
        let at = pe + 24 + size + 40 * i;
        if image.get(at..at + wanted.len()) == Some(wanted) {
            let start = u32_at(image, at + 12)? as usize;
            let end = start
                .checked_add(u32_at(image, at + 8)? as usize)
                .ok_or("Размер секции engine.dll превышен.")?;
            if start < end && end <= image.len() {
                return Ok(start..end);
            }
        }
    }
    Err("Не найдена секция engine.dll.".into())
}
fn unique(data: &[u8], pattern: &[u8], mask: &[u8]) -> Result<usize, String> {
    let mut matches = data
        .windows(pattern.len())
        .enumerate()
        .filter(|(_, bytes)| {
            bytes
                .iter()
                .zip(pattern)
                .zip(mask)
                .all(|((&a, &b), &m)| m == b'?' || a == b)
        });
    let at = matches
        .next()
        .map(|(at, _)| at)
        .ok_or("Сигнатура протокола не найдена: версия GMod не поддерживается.")?;
    if matches.next().is_some() {
        return Err("Сигнатура протокола неоднозначна; изменения отменены.".into());
    }
    Ok(at)
}
fn anchored(
    image: &[u8],
    text: &std::ops::Range<usize>,
    anchor: &[u8],
    pattern: &[u8],
) -> Result<usize, String> {
    let strings = section(image, b".rdata")?;
    let target =
        strings.start + unique(&image[strings.clone()], anchor, &vec![b'x'; anchor.len()])?;
    let mut matches = Vec::new();
    for at in text.start..text.end.saturating_sub(7) {
        if matches!(image[at], 0x48 | 0x4c) && image[at + 1] == 0x8d && image[at + 2] & 0xc7 == 5 {
            let disp = i32::from_le_bytes(image[at + 3..at + 7].try_into().unwrap());
            if (at as i64 + 7 + i64::from(disp)) == target as i64 {
                let begin = at.saturating_sub(64).max(text.start);
                for offset in begin..at.saturating_sub(pattern.len() - 1) {
                    if image.get(offset..offset + pattern.len()) == Some(pattern) {
                        matches.push(offset);
                    }
                }
            }
        }
    }
    matches.sort_unstable();
    matches.dedup();
    if matches.len() != 1 {
        return Err("Не найдена однозначная проверка версии engine.dll.".into());
    }
    Ok(matches[0])
}

fn discover(image: &[u8]) -> Result<Vec<Patch>, String> {
    let text = section(image, b".text")?;
    let code = &image[text.clone()];
    let connect = text.start + unique(code, CONNECT, b"xxxxxxxxx?x????xxxxx")? + 16;
    let host = text.start + unique(code, HOST, b"xxxxxxxxxxxxxxxx")? + 3;
    let check = text.start + unique(code, CHECK, b"xxxxx?xxx????xxxxxxxxx?xxxxx?")?;
    let server = anchored(
        image,
        &text,
        b"#GameUI_ServerRejectOldProtocol\0",
        b"\x41\x83\xf8\x18\x74",
    )? + 3;
    let status = anchored(
        image,
        &text,
        b"Protocol version %i\nNetwork version %s",
        b"\xba\x18\0\0\0",
    )? + 1;
    let before = image
        .get(check..check + 29)
        .ok_or("Неполная проверка протокола.")?;
    let accept = before[5]
        .checked_sub(6)
        .ok_or("Неизвестный переход проверки протокола.")?;
    let reject = before[22]
        .checked_add(9)
        .filter(|x| *x <= 127)
        .ok_or("Неизвестный переход отказа.")?;
    let mut after = vec![0x90; 29];
    after[..14].copy_from_slice(&[
        0x83, 0x7e, 0x20, 0xff, 0x74, before[5], 0x83, 0x7e, 0x20, 0x18, 0x74, accept, 0xeb, reject,
    ]);
    let mut patches = vec![Patch {
        offset: check,
        before: before.to_vec(),
        after,
    }];
    for (offset, len) in [(connect, 4), (host, 4), (server, 1), (status, 4)] {
        patches.push(Patch {
            offset,
            before: image[offset..offset + len].to_vec(),
            after: vec![0xff; len],
        });
    }
    Ok(patches)
}

fn read_memory(base: usize, target: &mut [u8]) -> bool {
    let mut read = 0;
    unsafe {
        ReadProcessMemory(
            GetCurrentProcess(),
            base as *const c_void,
            target.as_mut_ptr().cast(),
            target.len(),
            &mut read,
        ) != 0
            && read == target.len()
    }
}

fn loaded_image() -> Result<(usize, Vec<u8>), String> {
    let mut module = std::ptr::null_mut();
    // Закрепляем существующий движок до завершения процесса, не загружаем другую DLL.
    if unsafe { GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_PIN, w!("engine.dll"), &mut module) }
        == 0
    {
        return Err("Движок Garry’s Mod ещё не загружен.".into());
    }
    let base = module as usize;
    let mut header = vec![0u8; 4096];
    if !read_memory(base, &mut header) {
        return Err("Заголовок engine.dll недоступен.".into());
    }
    let pe = u32_at(&header, 0x3c)? as usize;
    let size = u32_at(&header, pe + 24 + 56)? as usize;
    if !(4096..=128 * 1024 * 1024).contains(&size) {
        return Err("Некорректный размер engine.dll.".into());
    }
    let mut image = vec![0; size];
    image[..4096].copy_from_slice(&header);
    for name in [b".text".as_slice(), b".rdata".as_slice()] {
        let range = section(&image, name)?;
        if !read_memory(base + range.start, &mut image[range]) {
            return Err("Секция engine.dll недоступна для чтения.".into());
        }
    }
    Ok((base, image))
}

struct Thread {
    handle: HANDLE,
    suspended: bool,
}
struct Threads {
    handles: Vec<Thread>,
}
// Windows x64 требует 16 байт; CONTEXT в windows-sys выровнен только на 8.
#[repr(C, align(16))]
struct ThreadContext(CONTEXT);

impl Threads {
    fn open() -> Result<Self, String> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if snapshot == INVALID_HANDLE_VALUE {
            return Err("Не удалось проверить потоки движка.".into());
        }
        let mut entry: THREADENTRY32 = unsafe { std::mem::zeroed() };
        entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
        let mut result = Self {
            handles: Vec::new(),
        };
        let pid = unsafe { GetCurrentProcessId() };
        let own = unsafe { GetCurrentThreadId() };
        let mut found = unsafe { Thread32First(snapshot, &mut entry) } != 0;
        let mut failed = false;
        while found {
            if entry.th32OwnerProcessID == pid && entry.th32ThreadID != own {
                let handle = unsafe {
                    OpenThread(
                        THREAD_GET_CONTEXT
                            | THREAD_SUSPEND_RESUME
                            | THREAD_QUERY_INFORMATION
                            | 0x00100000,
                        0,
                        entry.th32ThreadID,
                    )
                };
                if handle.is_null() {
                    if unsafe { GetLastError() } != ERROR_INVALID_PARAMETER {
                        failed = true;
                        break;
                    }
                } else {
                    result.handles.push(Thread {
                        handle,
                        suspended: false,
                    });
                }
            }
            found = unsafe { Thread32Next(snapshot, &mut entry) } != 0;
        }
        if !failed && unsafe { GetLastError() } != ERROR_NO_MORE_FILES {
            failed = true;
        }
        unsafe {
            CloseHandle(snapshot);
        }
        if failed {
            Err("Не удалось открыть поток движка; протокол не изменён.".into())
        } else {
            Ok(result)
        }
    }
    fn resume(&mut self) -> bool {
        let mut ok = true;
        for item in &mut self.handles {
            if !item.suspended {
                continue;
            }
            if unsafe { ResumeThread(item.handle) } == u32::MAX
                && unsafe { WaitForSingleObject(item.handle, 0) } != WAIT_OBJECT_0
            {
                ok = false;
            } else {
                item.suspended = false;
            }
        }
        ok
    }
}
impl Drop for Threads {
    fn drop(&mut self) {
        self.resume();
        for item in &self.handles {
            unsafe {
                CloseHandle(item.handle);
            }
        }
    }
}

// В приостановленном участке нет аллокаций, форматирования, ожиданий и Rust-блокировок.
unsafe fn write_raw(address: usize, bytes: &[u8], protection: u32) -> bool {
    let address = address as *mut c_void;
    let mut previous = 0;
    if unsafe { VirtualProtect(address, bytes.len(), PAGE_EXECUTE_READWRITE, &mut previous) } == 0 {
        return false;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), address.cast(), bytes.len());
    }
    let mut ignored = 0;
    let restored = unsafe { VirtualProtect(address, bytes.len(), protection, &mut ignored) } != 0;
    let flushed = unsafe { FlushInstructionCache(GetCurrentProcess(), address, bytes.len()) } != 0;
    restored && flushed
}

fn transaction(state: &mut Active, enabled: bool) -> Result<(), String> {
    let mut threads = Threads::open()?;
    // Снимки и сведения о защите готовы до приостановки чужих потоков.
    let mut snapshots = [[0u8; 32]; 5];
    let mut protections = [0u32; 5];
    let code = (|| {
        for item in &mut threads.handles {
            if unsafe { SuspendThread(item.handle) } == u32::MAX {
                if unsafe { WaitForSingleObject(item.handle, 0) } == WAIT_OBJECT_0 {
                    continue;
                }
                return 1;
            }
            item.suspended = true;
            let mut context = ThreadContext(unsafe { std::mem::zeroed() });
            context.0.ContextFlags = CONTEXT_CONTROL_AMD64;
            if unsafe { GetThreadContext(item.handle, &mut context.0) } == 0 {
                if unsafe { WaitForSingleObject(item.handle, 0) } == WAIT_OBJECT_0 {
                    continue;
                }
                return 1;
            }
            if state.patches.iter().any(|patch| {
                let begin = state.base + patch.offset;
                let end = begin + patch.after.len();
                (context.0.Rip as usize) >= begin.saturating_sub(15) && (context.0.Rip as usize) < end
            }) {
                return 2;
            }
        }
        for (i, patch) in state.patches.iter().enumerate() {
            let bytes = &mut snapshots[i][..patch.before.len()];
            if !read_memory(state.base + patch.offset, bytes) {
                return 3;
            }
            if bytes != patch.before && bytes != patch.after {
                return 3;
            }
            let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
            if unsafe {
                VirtualQuery(
                    (state.base + patch.offset) as *const c_void,
                    &mut info,
                    std::mem::size_of_val(&info),
                )
            } == 0
            {
                return 3;
            }
            protections[i] = info.Protect;
        }
        for (i, patch) in state.patches.iter().enumerate() {
            let wanted = if enabled { &patch.after } else { &patch.before };
            #[cfg(test)]
            let injected = FAIL_WRITE
                .compare_exchange(
                    i as isize,
                    -1,
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                )
                .is_ok();
            #[cfg(not(test))]
            let injected = false;
            if injected || !unsafe { write_raw(state.base + patch.offset, wanted, protections[i]) }
            {
                let mut restored = true;
                for j in (0..=i).rev() {
                    let previous = &state.patches[j];
                    restored &= unsafe {
                        write_raw(
                            state.base + previous.offset,
                            &snapshots[j][..previous.before.len()],
                            protections[j],
                        )
                    };
                }
                state.uncertain |= !restored;
                return if restored { 4 } else { 5 };
            }
        }
        state.enabled = enabled;
        state.uncertain = false;
        0
    })();
    let resumed = threads.resume();
    if !resumed {
        state.uncertain = true;
        return Err("Не завершено возобновление потоков движка; перезапустите игру.".into());
    }
    match code {
        0 => Ok(()),
        1 => Err("Не удалось приостановить и проверить потоки; протокол не изменён.".into()),
        2 => Err("Движок выполняет подключение. Повторите после возврата в меню.".into()),
        3 => Err("Память протокола изменилась другим модулем; изменения отменены.".into()),
        4 => Err("Запись протокола не удалась; исходное состояние восстановлено.".into()),
        _ => Err("Не завершён откат протокола; закройте Garry’s Mod перед продолжением.".into()),
    }
}

pub fn set(enabled: bool) -> Result<(), String> {
    let mut active = ACTIVE
        .lock()
        .map_err(|_| "Состояние протокола недоступно.")?;
    if active.is_none() {
        if !enabled {
            return Ok(());
        }
        let (base, image) = loaded_image()?;
        let patches = discover(&image)?;
        *active = Some(Active {
            base,
            patches,
            enabled: false,
            uncertain: false,
        });
    }
    let state = active.as_mut().unwrap();
    if state.uncertain {
        return Err(
            "Не завершено восстановление памяти движка. Закройте и запустите Garry’s Mod заново."
                .into(),
        );
    }
    if state.enabled == enabled && !state.uncertain {
        return Ok(());
    }
    transaction(state, enabled)
}

pub fn verify_supported() -> Result<(), String> {
    let active = ACTIVE
        .lock()
        .map_err(|_| "Состояние протокола недоступно.")?;
    if let Some(state) = active.as_ref() {
        return if state.uncertain {
            Err("Перезапустите Garry’s Mod после сбоя восстановления протокола.".into())
        } else {
            Ok(())
        };
    }
    let (_, image) = loaded_image()?;
    discover(&image).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_and_ambiguous_images_are_rejected() {
        assert!(discover(&[0; 4096]).is_err());
        assert!(unique(b"aaaa", b"aa", b"xx").is_err());
        assert_eq!(unique(b"abce", b"a\0c", b"x?x").unwrap(), 0);
    }
    #[test]
    #[ignore = "Проверка установленной Steam DLL на машине разработчика"]
    fn installed_stock_engine_has_all_five_unique_sites() {
        // Проверка фактической Steam DLL без загрузки/записи в неё; отдельная от runtime.
        let path = std::path::Path::new(
            r"D:\SteamLibrary\steamapps\common\GarrysMod\bin\win64\engine.dll",
        );
        let file = std::fs::read(path).unwrap();
        let pe = u32_at(&file, 0x3c).unwrap() as usize;
        let mut image = vec![0; u32_at(&file, pe + 24 + 56).unwrap() as usize];
        image[..4096].copy_from_slice(&file[..4096]);
        let count = u16::from_le_bytes(file[pe + 6..pe + 8].try_into().unwrap()) as usize;
        let opt = u16::from_le_bytes(file[pe + 20..pe + 22].try_into().unwrap()) as usize;
        for i in 0..count {
            let at = pe + 24 + opt + i * 40;
            let rva = u32_at(&file, at + 12).unwrap() as usize;
            let size = u32_at(&file, at + 16).unwrap() as usize;
            let raw = u32_at(&file, at + 20).unwrap() as usize;
            image[rva..rva + size].copy_from_slice(&file[raw..raw + size]);
        }
        let patches = discover(&image).unwrap();
        assert_eq!(patches.len(), 5);
        let before = image.clone();
        for p in &patches {
            image[p.offset..p.offset + p.after.len()].copy_from_slice(&p.after);
        }
        let check = &patches[0];
        assert_eq!(check.after[3], 255);
        assert_eq!(check.after[9], 24);
        for p in patches.iter().rev() {
            image[p.offset..p.offset + p.before.len()].copy_from_slice(&p.before);
        }
        assert_eq!(image, before);
    }

    #[test]
    fn executable_page_round_trip_and_mid_write_failure_restore_original_state() {
        use windows_sys::Win32::System::Memory::{
            VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READ,
            PAGE_READWRITE,
        };
        let page = unsafe {
            VirtualAlloc(
                std::ptr::null(),
                4096,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        } as usize;
        assert_ne!(page, 0);
        let patches = (0..5)
            .map(|i| Patch {
                offset: i * 64,
                before: vec![24; 4],
                after: vec![255; 4],
            })
            .collect::<Vec<_>>();
        for patch in &patches {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    patch.before.as_ptr(),
                    (page + patch.offset) as *mut u8,
                    4,
                );
            }
        }
        let mut old = 0;
        assert_ne!(
            unsafe { VirtualProtect(page as *mut c_void, 4096, PAGE_EXECUTE_READ, &mut old) },
            0
        );
        let mut state = Active {
            base: page,
            patches,
            enabled: false,
            uncertain: false,
        };
        FAIL_WRITE.store(2, std::sync::atomic::Ordering::SeqCst);
        assert!(transaction(&mut state, true).is_err());
        assert!(!state.enabled && !state.uncertain);
        for patch in &state.patches {
            let mut bytes = [0; 4];
            assert!(read_memory(page + patch.offset, &mut bytes));
            assert_eq!(bytes, [24; 4]);
        }
        transaction(&mut state, true).unwrap();
        assert!(state.enabled);
        for patch in &state.patches {
            let mut bytes = [0; 4];
            assert!(read_memory(page + patch.offset, &mut bytes));
            assert_eq!(bytes, [255; 4]);
        }
        transaction(&mut state, false).unwrap();
        assert!(!state.enabled);
        for patch in &state.patches {
            let mut bytes = [0; 4];
            assert!(read_memory(page + patch.offset, &mut bytes));
            assert_eq!(bytes, [24; 4]);
            let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
            assert_ne!(
                unsafe {
                    VirtualQuery(
                        (page + patch.offset) as *const c_void,
                        &mut info,
                        std::mem::size_of_val(&info),
                    )
                },
                0
            );
            assert_eq!(info.Protect, PAGE_EXECUTE_READ);
        }
        assert_ne!(
            unsafe { VirtualFree(page as *mut c_void, 0, MEM_RELEASE) },
            0
        );
    }
}
