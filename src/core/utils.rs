use std::{io::Write, path::Path, sync::atomic::{AtomicUsize, Ordering}, time::SystemTime};

use serde::Serialize;

use crate::{
    core::Gui,
    il2cpp::{
        ext::{Il2CppObjectExt, Il2CppStringExt},
        types::{Il2CppObject, Il2CppString}
    }
};

use super::{Error, Hachimi};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SendPtr(pub *mut Il2CppObject);

unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

pub fn char_to_utf16_index(text: &str, char_idx: usize) -> i32 {
    text.chars()
        .take(char_idx)
        .map(|c| c.len_utf16())
        .sum::<usize>() as i32
}

pub fn utf16_to_char_index(text: &str, utf16_idx: usize) -> usize {
    let mut current_utf16_pos = 0;
    let mut char_pos = 0;

    for c in text.chars() {
        if current_utf16_pos >= utf16_idx {
            break;
        }
        current_utf16_pos += c.len_utf16();
        char_pos += 1;
    }
    char_pos
}

pub fn str_visual_len(text: &str) -> usize {
    let mut count = 0;
    let mut is_in_tag = false;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '<' => is_in_tag = true,
            '>' => is_in_tag = false,
            '\\' => {
                if let Some(&'n') = chars.peek() {
                    chars.next();
                } else if !is_in_tag {
                    count += 1;
                }
            }
            _ => {
                if !is_in_tag {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn concat_unix_path(left: &str, right: &str) -> String {
    let mut str = String::with_capacity(left.len() + 1 + right.len());
    str.push_str(left);
    str.push_str("/");
    str.push_str(right);
    str
}

pub fn write_json_file<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Result<(), Error> {
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

// Checks for both \n and \\n
pub fn game_str_has_newline(string: *mut Il2CppString) -> bool {
    let mut got_backslash = false;
    for c in unsafe { (*string).as_utf16str().as_slice().iter() } {
        if got_backslash {
            if *c == 0x6E { // n
                return true;
            }
            got_backslash = false;
        }

        if *c == 0x0A { // newline
            return true;
        }
        else if *c == 0x5C { // backslash
            got_backslash = true; //
        }
    }

    false
}

pub fn scale_to_aspect_ratio(sizes: (i32, i32), aspect_ratio: f32, prefer_larger: bool) -> (i32, i32) {
    let (mut width, mut height) = sizes;
    let orig_aspect_ratio = width as f32 / height as f32;
    // Use original values if possible
    if (aspect_ratio - orig_aspect_ratio).abs() <= 0.001 {
        return sizes;
    }
    else if (aspect_ratio - 1.0/orig_aspect_ratio).abs() <= 0.001 {
        return (height, width);
    }

    let scale_by_height = if prefer_larger { height > width } else { width > height };
    if scale_by_height {
        width = (height as f32 * aspect_ratio).round() as i32;
        // height = height;
    }
    else {
        // width = width;
        height = (width as f32 / aspect_ratio).round() as i32;
    }

    (width, height)
}

pub fn get_file_modified_time<P: AsRef<Path>>(path: P) -> Option<SystemTime> {
    let metadata = std::fs::metadata(path).ok()?;
    if !metadata.is_file() { return None; }
    metadata.modified().ok()
}

pub fn get_data_path() -> String {
    #[cfg(target_os = "android")]
    {
        format!("/data/data/{}/files", Hachimi::instance().game.package_name)
    }

    #[cfg(target_os = "windows")]
    {
        use crate::{
            il2cpp::hook::UnityEngine_CoreModule::Application,
            windows::utils::{get_game_dir, get_exec_path}
        };

        let exec_name = get_exec_path().file_stem().unwrap_or_default().to_string_lossy().into_owned();
        let data_folder_name = format!("{}_Data", exec_name);

        let local_data_path = get_game_dir()
            .join(data_folder_name)
            .join("Persistent");

        let dir_ok = |path: &std::path::Path| {
            path.exists()
                && std::fs::read_dir(path)
                    .map(|mut d| d.next().is_some())
                    .unwrap_or(false)
                && path.join("master").join("master.mdb").exists()
        };

        if dir_ok(&local_data_path) {
            local_data_path.to_string_lossy().to_string()
        } else {
            unsafe { (*Application::get_persistentDataPath()).as_utf16str() }.to_string()
        }
    }
}

pub fn get_masterdb_path() -> String {
    info!("get_masterdb_path base: {}", get_data_path());
    format!("{}/master/master.mdb", get_data_path())
}

pub fn get_meta_path() -> String {
    #[cfg(target_os = "android")]
    {
        format!("{}/meta", get_data_path())
    }

    #[cfg(target_os = "windows")]
    {
        use crate::{
            core::game::Region,
            windows::utils::get_game_dir
        };

        let game = &Hachimi::instance().game;

        if game.region == Region::Taiwan {
            get_game_dir().join("meta").to_string_lossy().to_string()
        } else {
            std::path::PathBuf::from(get_data_path()).join("meta").to_string_lossy().to_string()
        }
    }
}

// Intentionally dumb png loader implementation that only loads RGBA8 images
pub fn notify_error(message: impl AsRef<str>) {
    let s = message.as_ref();
    error!("{}", s);
    if let Some(mutex) = Gui::instance() {
        mutex.lock().unwrap().show_notification(s);
    }
}

pub fn mul_int (base:i32, mult: f32) -> i32 {
    (base as f32 * mult).round() as i32
}

pub fn get_proc_address(handle: usize, name: &std::ffi::CStr) -> usize {
    #[cfg(target_os = "windows")]
    {
        crate::windows::utils::get_proc_address(windows::Win32::Foundation::HMODULE(handle as _), name)
    }
    #[cfg(target_os = "android")]
    {
        unsafe { libc::dlsym(handle as *mut libc::c_void, name.as_ptr()) as usize }
    }
}

static RACE_SEEK_STAGE: AtomicUsize = AtomicUsize::new(0);

pub fn race_seek_stage(stage: usize) {
    RACE_SEEK_STAGE.store(stage, Ordering::Release);
}

#[cfg(target_os = "windows")]
pub fn race_seek_seh<F: FnMut()>(mut f: F) -> bool {
    if let Err(e) = microseh::try_seh(|| f()) {
        let stage = RACE_SEEK_STAGE.load(Ordering::Acquire);
        error!(
            "[race slider] seek faulted at stage {}: {} at {:#x} (rip {:#x}), state reset, race left paused",
            stage,
            e.code(),
            e.address() as usize,
            e.registers().rip()
        );
        false
    } else {
        true
    }
}

#[cfg(target_os = "android")]
pub fn race_seek_seh<F: FnOnce()>(f: F) -> bool {
    f();
    true
}

pub fn clear_il2cpp_list(list: *mut Il2CppObject) {
    use crate::il2cpp::symbols::get_method_addr_cached;

    if list.is_null() { return; }

    let list_class = unsafe { (*list).klass() };
    if list_class.is_null() { return; }

    let clear_addr = get_method_addr_cached(list_class, c"Clear", 0);
    if clear_addr == 0 { return; }

    let clear: extern "C" fn(*mut Il2CppObject) = unsafe { std::mem::transmute(clear_addr) };
    clear(list);
}
