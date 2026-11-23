use serde_json::Value;

#[cfg(windows)]
pub fn get_json(base: &str, path: &str) -> Result<Value, String> {
    use std::ffi::c_void;
    use std::ptr;
    type Handle = *mut c_void;
    #[link(name = "winhttp")]
    extern "system" {
        fn WinHttpOpen(
            agent: *const u16,
            access: u32,
            proxy: *const u16,
            bypass: *const u16,
            flags: u32,
        ) -> Handle;
        fn WinHttpConnect(session: Handle, server: *const u16, port: u16, reserved: u32) -> Handle;
        fn WinHttpOpenRequest(
            connection: Handle,
            verb: *const u16,
            object: *const u16,
            version: *const u16,
            referer: *const u16,
            accept: *const *const u16,
            flags: u32,
        ) -> Handle;
        fn WinHttpSetTimeouts(
            handle: Handle,
            resolve: i32,
            connect: i32,
            send: i32,
            receive: i32,
        ) -> i32;
        fn WinHttpSetOption(handle: Handle, option: u32, buffer: *mut c_void, length: u32) -> i32;
        fn WinHttpSendRequest(
            handle: Handle,
            headers: *const u16,
            length: u32,
            optional: *const c_void,
            optional_length: u32,
            total: u32,
            context: usize,
        ) -> i32;
        fn WinHttpReceiveResponse(handle: Handle, reserved: *mut c_void) -> i32;
        fn WinHttpQueryHeaders(
            handle: Handle,
            level: u32,
            name: *const u16,
            buffer: *mut c_void,
            length: *mut u32,
            index: *mut u32,
        ) -> i32;
        fn WinHttpReadData(
            handle: Handle,
            buffer: *mut c_void,
            requested: u32,
            read: *mut u32,
        ) -> i32;
        fn WinHttpCloseHandle(handle: Handle) -> i32;
    }
    struct Owned(Handle);
    impl Drop for Owned {
        fn drop(&mut self) {
            unsafe { WinHttpCloseHandle(self.0) };
        }
    }
    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(Some(0)).collect()
    }
    fn failure() -> String {
        format!(
            "Публичный HTTPS API недоступен: {}",
            std::io::Error::last_os_error()
        )
    }
    fn own(handle: Handle) -> Result<Owned, String> {
        if handle.is_null() {
            Err(failure())
        } else {
            Ok(Owned(handle))
        }
    }
    let host = match base {
        "https://api.shinri.red" => "api.shinri.red",
        "https://api-fb1.shinri.red" => "api-fb1.shinri.red",
        "https://api.shinri-trial.com" => "api.shinri-trial.com",
        _ => return Err("API_BASE_NOT_ALLOWED".into()),
    };
    if !matches!(path, "/health" | "/games/servers/total-online") {
        return Err("API_PATH_NOT_ALLOWED".into());
    }
    unsafe {
        let session = own(WinHttpOpen(
            wide("PanopticonHub/0.1.0").as_ptr(),
            4,
            ptr::null(),
            ptr::null(),
            0,
        ))?;
        if WinHttpSetTimeouts(session.0, 800, 1000, 1000, 1500) == 0 {
            return Err(failure());
        }
        let connection = own(WinHttpConnect(session.0, wide(host).as_ptr(), 443, 0))?;
        let request = own(WinHttpOpenRequest(
            connection.0,
            wide("GET").as_ptr(),
            wide(path).as_ptr(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            0x00800000,
        ))?;
        // Перенаправления не расширяют список доверенных доменов.
        let mut redirect_policy = 0u32;
        if WinHttpSetOption(
            request.0,
            88,
            &mut redirect_policy as *mut _ as *mut c_void,
            4,
        ) == 0
        {
            return Err(failure());
        }
        if WinHttpSendRequest(
            request.0,
            wide("Accept: application/json\r\n").as_ptr(),
            u32::MAX,
            ptr::null(),
            0,
            0,
            0,
        ) == 0
            || WinHttpReceiveResponse(request.0, ptr::null_mut()) == 0
        {
            return Err(failure());
        }
        let mut status = 0u32;
        let mut length = 4u32;
        if WinHttpQueryHeaders(
            request.0,
            19 | 0x20000000,
            ptr::null(),
            &mut status as *mut _ as *mut c_void,
            &mut length,
            ptr::null_mut(),
        ) == 0
        {
            return Err(failure());
        }
        if status != 200 {
            return Err(format!("Публичный API вернул HTTP {status}"));
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(4);
        let mut bytes = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            if std::time::Instant::now() >= deadline {
                return Err("Истекло время получения публичного API".into());
            }
            let mut read = 0;
            if WinHttpReadData(
                request.0,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as u32,
                &mut read,
            ) == 0
            {
                return Err(failure());
            }
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..read as usize]);
            if bytes.len() > 1024 * 1024 {
                return Err("Ответ публичного API превышает 1 МиБ".into());
            }
        }
        serde_json::from_slice(&bytes)
            .map_err(|e| format!("Публичный API вернул некорректный JSON: {e}"))
    }
}

#[cfg(not(windows))]
pub fn get_json(_: &str, _: &str) -> Result<Value, String> {
    Err("Публичный HTTPS API поддерживается в Windows-сборке".into())
}
