use std::{ptr, sync::{atomic::{AtomicBool, Ordering}, Mutex, RwLock}};
use fnv::{FnvHashMap, FnvHashSet};
use once_cell::sync::Lazy;
use crate::{
    core::{utils::{get_masterdb_path, get_meta_path}, game::Region, Hachimi},
    il2cpp::{ext::{StringExt, Il2CppStringExt}, hook::{LibNative_Runtime::Sqlite3::{Connection, Query}, umamusume::SceneManager}}
};
use chrono::{Utc, Datelike};

pub static RETRIEVED_RAW_KEY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static AUTO_UNLOCK_NEXT_DB: AtomicBool = AtomicBool::new(false);
pub static META_DATA: Lazy<RwLock<MetaData>> = Lazy::new(|| RwLock::new(MetaData::default()));

// public API
#[derive(Default)]
pub struct CharacterData {
    pub chara_ids: FnvHashSet<i32>,
    pub chara_names: FnvHashMap<i32, String>
}

impl CharacterData {
    pub fn load_from_db() -> Self {
        let mut chara_ids = FnvHashSet::default();
        let mut chara_names = FnvHashMap::default();

        let db_path = get_masterdb_path();
        let conn = Connection::new();

        if Connection::Open(conn, db_path.to_il2cpp_string(), ptr::null_mut(), ptr::null_mut(), 0) {
            let sql = "SELECT C.id, T.text FROM chara_data AS C JOIN text_data AS T ON C.id = T.\"index\" WHERE T.id = 6";
            let query = Connection::Query(conn, sql.to_il2cpp_string());

            if !query.is_null() {
                while Query::Step(query) {
                    let id = Query::GetInt(query, 0);
                    let name_ptr = Query::GetText(query, 1);

                    if let Some(name) = unsafe { name_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                        chara_ids.insert(id);
                        chara_names.insert(id, name);
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        }

        CharacterData { chara_ids, chara_names }
    }

    pub fn exists(&self, id: i32) -> bool {
        self.chara_ids.contains(&id)
    }

    pub fn get_name(&self, id: i32) -> String {
        // fallback to default Japanese name from mdb
        if let Some(name) = self.chara_names.get(&id) {
            return name.clone();
        }

        // unknown character name
        "???".to_string()
    }
}

// untranslated skill info
#[derive(Default)]
pub struct SkillInfo {
    pub skill_names: FnvHashMap<i32, String>,
    pub skill_descs: FnvHashMap<i32, String>,
}

impl SkillInfo {
    pub fn load_from_db() -> Self {
        let mut skill_names = FnvHashMap::default();
        let mut skill_descs = FnvHashMap::default();

        let db_path = get_masterdb_path();
        let conn = Connection::new();

        if Connection::Open(conn, db_path.to_il2cpp_string(), ptr::null_mut(), ptr::null_mut(), 0) {
            // category 47 = names, 48 = descriptions
            let sql = "SELECT \"index\", text, id FROM text_data WHERE id IN (47, 48)";
            let query = Connection::Query(conn, sql.to_il2cpp_string());

            if !query.is_null() {
                while Query::Step(query) {
                    let index = Query::GetInt(query, 0);
                    let text_ptr = Query::GetText(query, 1);
                    let category = Query::GetInt(query, 2);

                    if let Some(text) = unsafe { text_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                        match category {
                            47 => skill_names.insert(index, text),
                            48 => skill_descs.insert(index, text),
                            _ => None,
                        };
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        }

        SkillInfo { skill_names, skill_descs }
    }

    pub fn get_name(&self, id: i32) -> String {
        if let Some(name) = self.skill_names.get(&id) {
            return name.clone();
        }

        // unknown skill name
        "???".to_string()
    }

    pub fn get_desc(&self, id: i32) -> String {
        if let Some(desc) = self.skill_descs.get(&id) {
            return desc.clone();
        }

        // unknown skill desc
        "???".to_string()
    }
}

#[derive(Default)]
pub struct MetaData {
    pub logical_name_to_hash: FnvHashMap<String, String>,
}

impl MetaData {
    pub fn get_hash(logical_name: &str) -> Option<String> {
        {
            let meta_read = META_DATA.read().unwrap();
            if !meta_read.logical_name_to_hash.is_empty() {
                return meta_read.logical_name_to_hash.get(logical_name).cloned();
            }
        }

        let mut meta_write = META_DATA.write().unwrap();

        if meta_write.logical_name_to_hash.is_empty() {
            if RETRIEVED_RAW_KEY.lock().unwrap().is_empty() {
                return None;
            }
            let loaded = Self::load_from_db();
            meta_write.logical_name_to_hash = loaded.logical_name_to_hash;
        }

        meta_write.logical_name_to_hash.get(logical_name).cloned()
    }

    fn load_from_db() -> Self {
        let mut logical_name_to_hash = FnvHashMap::default();

        let db_path_str = get_meta_path();

        let conn = Connection::new();

        if Hachimi::instance().game.region == Region::Japan {
            AUTO_UNLOCK_NEXT_DB.store(true, Ordering::Relaxed);
        }

        if Connection::Open(conn, db_path_str.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
            let sql = "SELECT n, h FROM a";
            let query = Connection::Query(conn, sql.to_il2cpp_string());

            if !query.is_null() {
                while Query::Step(query) {
                    let path_ptr = Query::GetText(query, 0);
                    let hash_ptr = Query::GetText(query, 1);

                    if let (Some(path_str), Some(hash_str)) = (
                        unsafe { path_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()),
                        unsafe { hash_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()),
                    ) {
                        let logical_name = if let Some(idx) = path_str.rfind('/') {
                            format!("{}.a", &path_str[idx + 1..])
                        } else {
                            format!("{}.a", path_str)
                        };

                        logical_name_to_hash.insert(logical_name, hash_str);
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        } else {
            error!("Failed to open meta database at: {}", db_path_str);
        }

        MetaData { logical_name_to_hash }
    }
}

fn get_single_column_int(sql: &str) -> Vec<i32> {
    let mut items = Vec::new();
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
        let query = Connection::Query(conn, sql.to_il2cpp_string());
        if !query.is_null() {
            while Query::Step(query) {
                items.push(Query::GetInt(query, 0));
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    items
}

pub fn get_all_chara_ids() -> Vec<i32> {
    get_single_column_int("SELECT id FROM chara_data")
}

pub fn get_all_dress_ids() -> Vec<i32> {
    get_single_column_int("SELECT id FROM dress_data")
}

pub fn get_all_music_ids() -> Vec<i32> {
    get_single_column_int("SELECT music_id FROM live_data")
}

pub fn get_all_mob_ids() -> Vec<i32> {
    get_single_column_int("SELECT mob_id FROM mob_data WHERE use_live = 1")
}

pub fn get_default_dress_ids() -> Vec<i32> {
    get_single_column_int("SELECT id FROM dress_data WHERE (condition_type = 1 OR condition_type = 4 OR condition_type = 5) AND use_live_theater = 1 AND id < 999")
}

pub fn get_all_cards() -> Vec<(i32, i32)> {
    let mut items = Vec::new();
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
        let query = Connection::Query(conn, "SELECT id, default_rarity FROM card_data WHERE id <= 999999".to_il2cpp_string());
        if !query.is_null() {
            while Query::Step(query) {
                items.push((Query::GetInt(query, 0), Query::GetInt(query, 1)));
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    items
}

pub fn get_master_text(category: i32, index: i32) -> Option<String> {
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
        let sql = format!("SELECT text FROM text_data WHERE \"category\" = {} AND \"index\" = {}", category, index);
        let query = Connection::Query(conn, sql.to_il2cpp_string());
        if !query.is_null() {
            if Query::Step(query) {
                let text_ptr = Query::GetText(query, 0);
                if let Some(text) = unsafe { text_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                    Query::Dispose(query);
                    Connection::CloseDB(conn);
                    return Some(text);
                }
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    None
}

pub fn get_jobs_info(reward_id: i32) -> Option<(i32, i32)> {
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
        let sql = format!("SELECT place_id, genre_id FROM jobs_reward WHERE \"id\" = {}", reward_id);
        let query = Connection::Query(conn, sql.to_il2cpp_string());
        if !query.is_null() {
            if Query::Step(query) {
                let place_id = Query::GetInt(query, 0);
                let genre_id = Query::GetInt(query, 1);
                Query::Dispose(query);
                Connection::CloseDB(conn);
                return Some((place_id, genre_id));
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    None
}

pub fn get_jobs_place_race_track_id(place_id: i32) -> Option<i32> {
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
        let sql = format!("SELECT race_track_id FROM jobs_place WHERE \"id\" = {}", place_id);
        let query = Connection::Query(conn, sql.to_il2cpp_string());
        if !query.is_null() {
            if Query::Step(query) {
                let track_id = Query::GetInt(query, 0);
                Query::Dispose(query);
                Connection::CloseDB(conn);
                return Some(track_id);
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    None
}

pub fn get_champions_resources() -> Vec<String> {
    let mut items = Vec::new();
    let db_path = get_masterdb_path();
    let conn = Connection::new();
    if Connection::Open(conn, db_path.to_il2cpp_string(), ptr::null_mut(), ptr::null_mut(), 0) {
        let sql = "SELECT t.text FROM champions_schedule c LEFT OUTER JOIN text_data t on t.category = 206 AND t.\"index\" = c.id GROUP BY c.resource_id";
        let query = Connection::Query(conn, sql.to_il2cpp_string());
        if !query.is_null() {
            while Query::Step(query) {
                let text_ptr = Query::GetText(query, 0);
                if let Some(text) = unsafe { text_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                    items.push(text);
                } else {
                    items.push(rust_i18n::t!("unknown").into_owned());
                }
            }
            Query::Dispose(query);
        }
        Connection::CloseDB(conn);
    }
    items
}

pub fn get_champions_live_max_year() -> i32 {
    let mut max_year = Utc::now().year(); // fallback to the current year since it's guaranteed to have textures
    if !SceneManager::is_home_init() { return max_year; }
    let db_path_str = get_meta_path();

    let conn = Connection::new();
    if Hachimi::instance().game.region == Region::Japan {
        AUTO_UNLOCK_NEXT_DB.store(true, Ordering::Relaxed);
    }
    if Connection::Open(conn, db_path_str.to_il2cpp_string(), ptr::null_mut(), ptr::null_mut(), 0) {
        let sql = "SELECT n FROM a WHERE n LIKE 'live/image/champions/tex_championslive_year_%'";
        let query = Connection::Query(conn, sql.to_il2cpp_string());

        if !query.is_null() {
            let mut max_idx = -1;
            while Query::Step(query) {
                let text_ptr = Query::GetText(query, 0);
                if let Some(text) = unsafe { text_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                    if let Some(idx_str) = text.strip_prefix("live/image/champions/tex_championslive_year_") {
                        if let Ok(idx) = idx_str.parse::<i32>() {
                            max_idx = max_idx.max(idx);
                        }
                    }
                }
            }
            Query::Dispose(query);
            if max_idx >= 0 {
                max_year = 2022 + max_idx;
            }
        }
        Connection::CloseDB(conn);
    }
    max_year
}
