use crate::export_indices::*;
use crate::{ORIGINAL_FUNCS, ORIG_DLL_HANDLE};
use std::ffi::CString;
use winapi::{
    shared::minwindef::{FARPROC, HMODULE},
    um::libloaderapi::GetProcAddress,
};

/// Loads up the address of the original function in the given module
unsafe fn load_dll_func(index: usize, h_module: HMODULE, func: &str) {
    let func_c_string = CString::new(func).unwrap();
    let proc_address: FARPROC = GetProcAddress(h_module, func_c_string.as_ptr());
    ORIGINAL_FUNCS[index] = proc_address;
    println!("[0x{:016x}] Loaded {}", proc_address as u64, func);
}

/// Loads the original DLL functions for later use
pub unsafe fn load_dll_funcs() {
    println!("Loading original DLL functions");
    if ORIG_DLL_HANDLE.is_none() {
        eprintln!("Original DLL handle is none. Cannot load original DLL funcs");
        return;
    }
    let dll_handle = ORIG_DLL_HANDLE.unwrap();
    load_dll_func(Index_BASS_Apply3D, dll_handle, "BASS_Apply3D");
    load_dll_func(Index_BASS_ChannelBytes2Seconds, dll_handle, "BASS_ChannelBytes2Seconds");
    load_dll_func(Index_BASS_ChannelFlags, dll_handle, "BASS_ChannelFlags");
    load_dll_func(Index_BASS_ChannelFree, dll_handle, "BASS_ChannelFree");
    load_dll_func(Index_BASS_ChannelGet3DAttributes, dll_handle, "BASS_ChannelGet3DAttributes");
    load_dll_func(Index_BASS_ChannelGet3DPosition, dll_handle, "BASS_ChannelGet3DPosition");
    load_dll_func(Index_BASS_ChannelGetAttribute, dll_handle, "BASS_ChannelGetAttribute");
    load_dll_func(Index_BASS_ChannelGetAttributeEx, dll_handle, "BASS_ChannelGetAttributeEx");
    load_dll_func(Index_BASS_ChannelGetData, dll_handle, "BASS_ChannelGetData");
    load_dll_func(Index_BASS_ChannelGetDevice, dll_handle, "BASS_ChannelGetDevice");
    load_dll_func(Index_BASS_ChannelGetInfo, dll_handle, "BASS_ChannelGetInfo");
    load_dll_func(Index_BASS_ChannelGetLength, dll_handle, "BASS_ChannelGetLength");
    load_dll_func(Index_BASS_ChannelGetLevel, dll_handle, "BASS_ChannelGetLevel");
    load_dll_func(Index_BASS_ChannelGetLevelEx, dll_handle, "BASS_ChannelGetLevelEx");
    load_dll_func(Index_BASS_ChannelGetPosition, dll_handle, "BASS_ChannelGetPosition");
    load_dll_func(Index_BASS_ChannelGetTags, dll_handle, "BASS_ChannelGetTags");
    load_dll_func(Index_BASS_ChannelIsActive, dll_handle, "BASS_ChannelIsActive");
    load_dll_func(Index_BASS_ChannelIsSliding, dll_handle, "BASS_ChannelIsSliding");
    load_dll_func(Index_BASS_ChannelLock, dll_handle, "BASS_ChannelLock");
    load_dll_func(Index_BASS_ChannelPause, dll_handle, "BASS_ChannelPause");
    load_dll_func(Index_BASS_ChannelPlay, dll_handle, "BASS_ChannelPlay");
    load_dll_func(Index_BASS_ChannelRemoveDSP, dll_handle, "BASS_ChannelRemoveDSP");
    load_dll_func(Index_BASS_ChannelRemoveFX, dll_handle, "BASS_ChannelRemoveFX");
    load_dll_func(Index_BASS_ChannelRemoveLink, dll_handle, "BASS_ChannelRemoveLink");
    load_dll_func(Index_BASS_ChannelRemoveSync, dll_handle, "BASS_ChannelRemoveSync");
    load_dll_func(Index_BASS_ChannelSeconds2Bytes, dll_handle, "BASS_ChannelSeconds2Bytes");
    load_dll_func(Index_BASS_ChannelSet3DAttributes, dll_handle, "BASS_ChannelSet3DAttributes");
    load_dll_func(Index_BASS_ChannelSet3DPosition, dll_handle, "BASS_ChannelSet3DPosition");
    load_dll_func(Index_BASS_ChannelSetAttribute, dll_handle, "BASS_ChannelSetAttribute");
    load_dll_func(Index_BASS_ChannelSetAttributeEx, dll_handle, "BASS_ChannelSetAttributeEx");
    load_dll_func(Index_BASS_ChannelSetDSP, dll_handle, "BASS_ChannelSetDSP");
    load_dll_func(Index_BASS_ChannelSetDevice, dll_handle, "BASS_ChannelSetDevice");
    load_dll_func(Index_BASS_ChannelSetFX, dll_handle, "BASS_ChannelSetFX");
    load_dll_func(Index_BASS_ChannelSetLink, dll_handle, "BASS_ChannelSetLink");
    load_dll_func(Index_BASS_ChannelSetPosition, dll_handle, "BASS_ChannelSetPosition");
    load_dll_func(Index_BASS_ChannelSetSync, dll_handle, "BASS_ChannelSetSync");
    load_dll_func(Index_BASS_ChannelSlideAttribute, dll_handle, "BASS_ChannelSlideAttribute");
    load_dll_func(Index_BASS_ChannelStop, dll_handle, "BASS_ChannelStop");
    load_dll_func(Index_BASS_ChannelUpdate, dll_handle, "BASS_ChannelUpdate");
    load_dll_func(Index_BASS_ErrorGetCode, dll_handle, "BASS_ErrorGetCode");
    load_dll_func(Index_BASS_FXGetParameters, dll_handle, "BASS_FXGetParameters");
    load_dll_func(Index_BASS_FXReset, dll_handle, "BASS_FXReset");
    load_dll_func(Index_BASS_FXSetParameters, dll_handle, "BASS_FXSetParameters");
    load_dll_func(Index_BASS_FXSetPriority, dll_handle, "BASS_FXSetPriority");
    load_dll_func(Index_BASS_Free, dll_handle, "BASS_Free");
    load_dll_func(Index_BASS_Get3DFactors, dll_handle, "BASS_Get3DFactors");
    load_dll_func(Index_BASS_Get3DPosition, dll_handle, "BASS_Get3DPosition");
    load_dll_func(Index_BASS_GetCPU, dll_handle, "BASS_GetCPU");
    load_dll_func(Index_BASS_GetConfig, dll_handle, "BASS_GetConfig");
    load_dll_func(Index_BASS_GetConfigPtr, dll_handle, "BASS_GetConfigPtr");
    load_dll_func(Index_BASS_GetDSoundObject, dll_handle, "BASS_GetDSoundObject");
    load_dll_func(Index_BASS_GetDevice, dll_handle, "BASS_GetDevice");
    load_dll_func(Index_BASS_GetDeviceInfo, dll_handle, "BASS_GetDeviceInfo");
    load_dll_func(Index_BASS_GetEAXParameters, dll_handle, "BASS_GetEAXParameters");
    load_dll_func(Index_BASS_GetInfo, dll_handle, "BASS_GetInfo");
    load_dll_func(Index_BASS_GetVersion, dll_handle, "BASS_GetVersion");
    load_dll_func(Index_BASS_GetVolume, dll_handle, "BASS_GetVolume");
    load_dll_func(Index_BASS_Init, dll_handle, "BASS_Init");
    load_dll_func(Index_BASS_IsStarted, dll_handle, "BASS_IsStarted");
    load_dll_func(Index_BASS_MusicFree, dll_handle, "BASS_MusicFree");
    load_dll_func(Index_BASS_MusicLoad, dll_handle, "BASS_MusicLoad");
    load_dll_func(Index_BASS_Pause, dll_handle, "BASS_Pause");
    load_dll_func(Index_BASS_PluginEnable, dll_handle, "BASS_PluginEnable");
    load_dll_func(Index_BASS_PluginFree, dll_handle, "BASS_PluginFree");
    load_dll_func(Index_BASS_PluginGetInfo, dll_handle, "BASS_PluginGetInfo");
    load_dll_func(Index_BASS_PluginLoad, dll_handle, "BASS_PluginLoad");
    load_dll_func(Index_BASS_RecordFree, dll_handle, "BASS_RecordFree");
    load_dll_func(Index_BASS_RecordGetDevice, dll_handle, "BASS_RecordGetDevice");
    load_dll_func(Index_BASS_RecordGetDeviceInfo, dll_handle, "BASS_RecordGetDeviceInfo");
    load_dll_func(Index_BASS_RecordGetInfo, dll_handle, "BASS_RecordGetInfo");
    load_dll_func(Index_BASS_RecordGetInput, dll_handle, "BASS_RecordGetInput");
    load_dll_func(Index_BASS_RecordGetInputName, dll_handle, "BASS_RecordGetInputName");
    load_dll_func(Index_BASS_RecordInit, dll_handle, "BASS_RecordInit");
    load_dll_func(Index_BASS_RecordSetDevice, dll_handle, "BASS_RecordSetDevice");
    load_dll_func(Index_BASS_RecordSetInput, dll_handle, "BASS_RecordSetInput");
    load_dll_func(Index_BASS_RecordStart, dll_handle, "BASS_RecordStart");
    load_dll_func(Index_BASS_SampleCreate, dll_handle, "BASS_SampleCreate");
    load_dll_func(Index_BASS_SampleFree, dll_handle, "BASS_SampleFree");
    load_dll_func(Index_BASS_SampleGetChannel, dll_handle, "BASS_SampleGetChannel");
    load_dll_func(Index_BASS_SampleGetChannels, dll_handle, "BASS_SampleGetChannels");
    load_dll_func(Index_BASS_SampleGetData, dll_handle, "BASS_SampleGetData");
    load_dll_func(Index_BASS_SampleGetInfo, dll_handle, "BASS_SampleGetInfo");
    load_dll_func(Index_BASS_SampleLoad, dll_handle, "BASS_SampleLoad");
    load_dll_func(Index_BASS_SampleSetData, dll_handle, "BASS_SampleSetData");
    load_dll_func(Index_BASS_SampleSetInfo, dll_handle, "BASS_SampleSetInfo");
    load_dll_func(Index_BASS_SampleStop, dll_handle, "BASS_SampleStop");
    load_dll_func(Index_BASS_Set3DFactors, dll_handle, "BASS_Set3DFactors");
    load_dll_func(Index_BASS_Set3DPosition, dll_handle, "BASS_Set3DPosition");
    load_dll_func(Index_BASS_SetConfig, dll_handle, "BASS_SetConfig");
    load_dll_func(Index_BASS_SetConfigPtr, dll_handle, "BASS_SetConfigPtr");
    load_dll_func(Index_BASS_SetDevice, dll_handle, "BASS_SetDevice");
    load_dll_func(Index_BASS_SetEAXParameters, dll_handle, "BASS_SetEAXParameters");
    load_dll_func(Index_BASS_SetVolume, dll_handle, "BASS_SetVolume");
    load_dll_func(Index_BASS_Start, dll_handle, "BASS_Start");
    load_dll_func(Index_BASS_Stop, dll_handle, "BASS_Stop");
    load_dll_func(Index_BASS_StreamCreate, dll_handle, "BASS_StreamCreate");
    load_dll_func(Index_BASS_StreamCreateFile, dll_handle, "BASS_StreamCreateFile");
    load_dll_func(Index_BASS_StreamCreateFileUser, dll_handle, "BASS_StreamCreateFileUser");
    load_dll_func(Index_BASS_StreamCreateURL, dll_handle, "BASS_StreamCreateURL");
    load_dll_func(Index_BASS_StreamFree, dll_handle, "BASS_StreamFree");
    load_dll_func(Index_BASS_StreamGetFilePosition, dll_handle, "BASS_StreamGetFilePosition");
    load_dll_func(Index_BASS_StreamPutData, dll_handle, "BASS_StreamPutData");
    load_dll_func(Index_BASS_StreamPutFileData, dll_handle, "BASS_StreamPutFileData");
    load_dll_func(Index_BASS_Update, dll_handle, "BASS_Update");
    load_dll_func(Index__, dll_handle, "_");
}
