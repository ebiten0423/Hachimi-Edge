use crate::il2cpp::{symbols::get_method_addr, types::*};

use super::AssetBundle;

type GetResultFn = extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetResult(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let asset = get_orig_fn!(GetResult, GetResultFn)(this);
    if AssetBundle::PENDING_REQUESTS.lock().unwrap().remove(&(this as usize)) {
        AssetBundle::on_LoadAsset(asset);
    }
    asset
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, UnityEngine, AssetBundleRequest);
    let addr = get_method_addr(AssetBundleRequest, c"GetResult", 0);
    new_hook!(addr, GetResult);
}
