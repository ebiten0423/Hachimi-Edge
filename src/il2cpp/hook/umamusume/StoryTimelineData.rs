use std::ptr::null_mut;

use crate::{core::Hachimi, il2cpp::{ext::Il2CppStringExt, hook::umamusume::{StoryTimelineBlockData, StoryTimelineCharaTrackData, StoryTimelineClipData, StoryTimelineTextClipData, StoryTimelineTrackData}, symbols::{get_field_from_name, get_field_object_value, get_field_value, set_field_value, IList}, types::*}};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass { unsafe { CLASS } }

static mut BLOCKLIST_FIELD: *mut FieldInfo = null_mut();
pub fn get_BlockList(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { BLOCKLIST_FIELD })
}

static mut TYPEWRITECOUNTPERSECOND_FIELD: *mut FieldInfo = null_mut();
fn get_TypewriteCountPerSecond(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { TYPEWRITECOUNTPERSECOND_FIELD })
}
fn set_TypewriteCountPerSecond(this: *mut Il2CppObject, value: i32) {
    set_field_value(this, unsafe { TYPEWRITECOUNTPERSECOND_FIELD }, &value);
}

static mut LENGTH_FIELD: *mut FieldInfo = null_mut();
fn set_Length(this: *mut Il2CppObject, value: i32) {
    set_field_value(this, unsafe { LENGTH_FIELD }, &value);
}

// Keep the independent story text speed control. No story text is replaced.
pub fn on_LoadAsset(this: *mut Il2CppObject) {
    let multiplier = Hachimi::instance().config.load().story_tcps_multiplier;
    if multiplier == 1.0 { return; }
    let tcps = (get_TypewriteCountPerSecond(this) as f32 * multiplier).round().max(1.0);
    set_TypewriteCountPerSecond(this, tcps as i32);
    if multiplier < 1.0 {
        adjust_clips_length_with_tcps(this, tcps);
    }
}

fn get_typewrite_length(text_len: usize, tcps: f32) -> i32 {
    (text_len as f32 / tcps * 30.0).round() as i32 // len / cps * fps
}

fn adjust_clips_length_with_tcps(this: *mut Il2CppObject, tcps: f32) {
    let Some(block_list) = IList::new(get_BlockList(this)) else {
        return;
    };
    let mut block_list_iter = block_list.iter();

    // First block is always empty, no need to adjust length
    let Some(first_block_data) = block_list_iter.next() else {
        return;
    };
    let mut total_len = StoryTimelineBlockData::get_BlockLength(first_block_data);

    for block_data in block_list_iter {
        let orig_block_len = StoryTimelineBlockData::get_BlockLength(block_data);
        let Some(clip_data) = StoryTimelineBlockData::get_text_clip(block_data) else {
            total_len += orig_block_len;
            continue;
        };
        let text = StoryTimelineTextClipData::get_Text(clip_data);

        total_len += if text.is_null() {
            orig_block_len
        }
        else {
            let orig_clip_len = StoryTimelineClipData::get_ClipLength(clip_data);
            let new_clip_len = get_typewrite_length(unsafe { (*text).as_utf16str().chars().count() }, tcps);

            if new_clip_len > orig_clip_len {
                apply_clip_length(clip_data, orig_clip_len, new_clip_len, block_data, orig_block_len)
            }
            else {
                orig_block_len
            }
        }
    }

    set_Length(this, total_len);
}

/// Returns new block length
fn apply_clip_length(
    clip_data: *mut Il2CppObject, orig_clip_len: i32, new_clip_len: i32,
    block_data: *mut Il2CppObject, orig_block_len: i32
) -> i32 {
    StoryTimelineClipData::set_ClipLength(clip_data, new_clip_len);
    let new_block_len = StoryTimelineClipData::get_StartFrame(clip_data) + new_clip_len + 1;
    StoryTimelineBlockData::set_BlockLength(block_data, new_block_len);

    let clip_len_diff = new_clip_len - orig_clip_len;

    // Adjust anim lengths
    if let Some(chara_track_list) = <IList>::new(StoryTimelineBlockData::get_CharacterTrackList(block_data)) {
        for chara_track_data in chara_track_list.iter() {
            for motion_track_data in StoryTimelineCharaTrackData::motion_track_data_values(chara_track_data) {
                let Some(clip_list) = <IList>::new(StoryTimelineTrackData::get_ClipList(motion_track_data)) else {
                    continue;
                };
                let Some(clip_data) = clip_list.get(clip_list.count() - 1) else {
                    continue;
                };

                let orig_motion_clip_len = StoryTimelineClipData::get_ClipLength(clip_data);
                let new_motion_clip_len = orig_motion_clip_len + clip_len_diff;
                StoryTimelineClipData::set_ClipLength(clip_data, new_motion_clip_len);
            }
        }
    }

    // Adjust screen effect lengths
    if let Some(se_track_list) = <IList>::new(StoryTimelineBlockData::get_ScreenEffectTrackList(block_data)) {
        for se_track_data in se_track_list.iter() {
            let Some(clip_list) = <IList>::new(StoryTimelineTrackData::get_ClipList(se_track_data)) else {
                continue;
            };
            let Some(clip_data) = clip_list.get(clip_list.count() - 1) else {
                continue;
            };

            let start_frame = StoryTimelineClipData::get_StartFrame(clip_data);
            let orig_se_clip_len = StoryTimelineClipData::get_ClipLength(clip_data);
            // if it extends to the end of the block
            if start_frame + orig_se_clip_len < orig_block_len {
                continue;
            }

            let new_se_clip_len = orig_se_clip_len + clip_len_diff;
            StoryTimelineClipData::set_ClipLength(clip_data, new_se_clip_len);
        }
    }

    new_block_len
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, StoryTimelineData);
    unsafe {
        CLASS = StoryTimelineData;
        BLOCKLIST_FIELD = get_field_from_name(StoryTimelineData, c"BlockList");
        TYPEWRITECOUNTPERSECOND_FIELD = get_field_from_name(StoryTimelineData, c"TypewriteCountPerSecond");
        LENGTH_FIELD = get_field_from_name(StoryTimelineData, c"Length");
    }
}
