use std::sync::Mutex;

use fnv::FnvHashSet;
use once_cell::sync::Lazy;

use crate::il2cpp::{api::il2cpp_resolve_icall, ext::Il2CppObjectExt, hook::umamusume::StoryTimelineData, types::*};

pub static PENDING_REQUESTS: Lazy<Mutex<FnvHashSet<usize>>> = Lazy::new(|| Mutex::new(FnvHashSet::default()));

pub fn on_LoadAsset(asset: *mut Il2CppObject) {
    if !asset.is_null() && unsafe { (*asset).klass() } == StoryTimelineData::class() {
        StoryTimelineData::on_LoadAsset(asset);
    }
}

type LoadAssetFn = extern "C" fn(*mut Il2CppObject, *mut Il2CppString, *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn LoadAsset_Internal(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    let asset = get_orig_fn!(LoadAsset_Internal, LoadAssetFn)(this, name, type_);
    on_LoadAsset(asset);
    asset
}

pub fn LoadAsset_Internal_orig(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    get_orig_fn!(LoadAsset_Internal, LoadAssetFn)(this, name, type_)
}

type LoadAssetAsyncFn = extern "C" fn(*mut Il2CppObject, *mut Il2CppString, *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn LoadAssetAsync_Internal(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    let request = get_orig_fn!(LoadAssetAsync_Internal, LoadAssetAsyncFn)(this, name, type_);
    if !request.is_null() {
        PENDING_REQUESTS.lock().unwrap().insert(request as usize);
    }
    request
}

pub fn init(_image: *const Il2CppImage) {
    let sync_addr = il2cpp_resolve_icall(c"UnityEngine.AssetBundle::LoadAsset_Internal(System.String,System.Type)".as_ptr());
    let async_addr = il2cpp_resolve_icall(c"UnityEngine.AssetBundle::LoadAssetAsync_Internal(System.String,System.Type)".as_ptr());
    new_hook!(sync_addr, LoadAsset_Internal);
    new_hook!(async_addr, LoadAssetAsync_Internal);
}
