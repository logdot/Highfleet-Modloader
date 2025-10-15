// Just proxied functions in this file
use proxygen_macros::forward;

#[forward]
#[export_name="BASS_Apply3D"]
pub extern "C" fn BASS_Apply3D() {}

#[forward]
#[export_name="BASS_ChannelBytes2Seconds"]
pub extern "C" fn BASS_ChannelBytes2Seconds() {}

#[forward]
#[export_name="BASS_ChannelFlags"]
pub extern "C" fn BASS_ChannelFlags() {}

#[forward]
#[export_name="BASS_ChannelFree"]
pub extern "C" fn BASS_ChannelFree() {}

#[forward]
#[export_name="BASS_ChannelGet3DAttributes"]
pub extern "C" fn BASS_ChannelGet3DAttributes() {}

#[forward]
#[export_name="BASS_ChannelGet3DPosition"]
pub extern "C" fn BASS_ChannelGet3DPosition() {}

#[forward]
#[export_name="BASS_ChannelGetAttribute"]
pub extern "C" fn BASS_ChannelGetAttribute() {}

#[forward]
#[export_name="BASS_ChannelGetAttributeEx"]
pub extern "C" fn BASS_ChannelGetAttributeEx() {}

#[forward]
#[export_name="BASS_ChannelGetData"]
pub extern "C" fn BASS_ChannelGetData() {}

#[forward]
#[export_name="BASS_ChannelGetDevice"]
pub extern "C" fn BASS_ChannelGetDevice() {}

#[forward]
#[export_name="BASS_ChannelGetInfo"]
pub extern "C" fn BASS_ChannelGetInfo() {}

#[forward]
#[export_name="BASS_ChannelGetLength"]
pub extern "C" fn BASS_ChannelGetLength() {}

#[forward]
#[export_name="BASS_ChannelGetLevel"]
pub extern "C" fn BASS_ChannelGetLevel() {}

#[forward]
#[export_name="BASS_ChannelGetLevelEx"]
pub extern "C" fn BASS_ChannelGetLevelEx() {}

#[forward]
#[export_name="BASS_ChannelGetPosition"]
pub extern "C" fn BASS_ChannelGetPosition() {}

#[forward]
#[export_name="BASS_ChannelGetTags"]
pub extern "C" fn BASS_ChannelGetTags() {}

#[forward]
#[export_name="BASS_ChannelIsActive"]
pub extern "C" fn BASS_ChannelIsActive() {}

#[forward]
#[export_name="BASS_ChannelIsSliding"]
pub extern "C" fn BASS_ChannelIsSliding() {}

#[forward]
#[export_name="BASS_ChannelLock"]
pub extern "C" fn BASS_ChannelLock() {}

#[forward]
#[export_name="BASS_ChannelPause"]
pub extern "C" fn BASS_ChannelPause() {}

#[forward]
#[export_name="BASS_ChannelPlay"]
pub extern "C" fn BASS_ChannelPlay() {}

#[forward]
#[export_name="BASS_ChannelRemoveDSP"]
pub extern "C" fn BASS_ChannelRemoveDSP() {}

#[forward]
#[export_name="BASS_ChannelRemoveFX"]
pub extern "C" fn BASS_ChannelRemoveFX() {}

#[forward]
#[export_name="BASS_ChannelRemoveLink"]
pub extern "C" fn BASS_ChannelRemoveLink() {}

#[forward]
#[export_name="BASS_ChannelRemoveSync"]
pub extern "C" fn BASS_ChannelRemoveSync() {}

#[forward]
#[export_name="BASS_ChannelSeconds2Bytes"]
pub extern "C" fn BASS_ChannelSeconds2Bytes() {}

#[forward]
#[export_name="BASS_ChannelSet3DAttributes"]
pub extern "C" fn BASS_ChannelSet3DAttributes() {}

#[forward]
#[export_name="BASS_ChannelSet3DPosition"]
pub extern "C" fn BASS_ChannelSet3DPosition() {}

#[forward]
#[export_name="BASS_ChannelSetAttribute"]
pub extern "C" fn BASS_ChannelSetAttribute() {}

#[forward]
#[export_name="BASS_ChannelSetAttributeEx"]
pub extern "C" fn BASS_ChannelSetAttributeEx() {}

#[forward]
#[export_name="BASS_ChannelSetDSP"]
pub extern "C" fn BASS_ChannelSetDSP() {}

#[forward]
#[export_name="BASS_ChannelSetDevice"]
pub extern "C" fn BASS_ChannelSetDevice() {}

#[forward]
#[export_name="BASS_ChannelSetFX"]
pub extern "C" fn BASS_ChannelSetFX() {}

#[forward]
#[export_name="BASS_ChannelSetLink"]
pub extern "C" fn BASS_ChannelSetLink() {}

#[forward]
#[export_name="BASS_ChannelSetPosition"]
pub extern "C" fn BASS_ChannelSetPosition() {}

#[forward]
#[export_name="BASS_ChannelSetSync"]
pub extern "C" fn BASS_ChannelSetSync() {}

#[forward]
#[export_name="BASS_ChannelSlideAttribute"]
pub extern "C" fn BASS_ChannelSlideAttribute() {}

#[forward]
#[export_name="BASS_ChannelStop"]
pub extern "C" fn BASS_ChannelStop() {}

#[forward]
#[export_name="BASS_ChannelUpdate"]
pub extern "C" fn BASS_ChannelUpdate() {}

#[forward]
#[export_name="BASS_ErrorGetCode"]
pub extern "C" fn BASS_ErrorGetCode() {}

#[forward]
#[export_name="BASS_FXGetParameters"]
pub extern "C" fn BASS_FXGetParameters() {}

#[forward]
#[export_name="BASS_FXReset"]
pub extern "C" fn BASS_FXReset() {}

#[forward]
#[export_name="BASS_FXSetParameters"]
pub extern "C" fn BASS_FXSetParameters() {}

#[forward]
#[export_name="BASS_FXSetPriority"]
pub extern "C" fn BASS_FXSetPriority() {}

#[forward]
#[export_name="BASS_Free"]
pub extern "C" fn BASS_Free() {}

#[forward]
#[export_name="BASS_Get3DFactors"]
pub extern "C" fn BASS_Get3DFactors() {}

#[forward]
#[export_name="BASS_Get3DPosition"]
pub extern "C" fn BASS_Get3DPosition() {}

#[forward]
#[export_name="BASS_GetCPU"]
pub extern "C" fn BASS_GetCPU() {}

#[forward]
#[export_name="BASS_GetConfig"]
pub extern "C" fn BASS_GetConfig() {}

#[forward]
#[export_name="BASS_GetConfigPtr"]
pub extern "C" fn BASS_GetConfigPtr() {}

#[forward]
#[export_name="BASS_GetDSoundObject"]
pub extern "C" fn BASS_GetDSoundObject() {}

#[forward]
#[export_name="BASS_GetDevice"]
pub extern "C" fn BASS_GetDevice() {}

#[forward]
#[export_name="BASS_GetDeviceInfo"]
pub extern "C" fn BASS_GetDeviceInfo() {}

#[forward]
#[export_name="BASS_GetEAXParameters"]
pub extern "C" fn BASS_GetEAXParameters() {}

#[forward]
#[export_name="BASS_GetInfo"]
pub extern "C" fn BASS_GetInfo() {}

#[forward]
#[export_name="BASS_GetVersion"]
pub extern "C" fn BASS_GetVersion() {}

#[forward]
#[export_name="BASS_GetVolume"]
pub extern "C" fn BASS_GetVolume() {}

#[forward]
#[export_name="BASS_Init"]
pub extern "C" fn BASS_Init() {}

#[forward]
#[export_name="BASS_IsStarted"]
pub extern "C" fn BASS_IsStarted() {}

#[forward]
#[export_name="BASS_MusicFree"]
pub extern "C" fn BASS_MusicFree() {}

#[forward]
#[export_name="BASS_MusicLoad"]
pub extern "C" fn BASS_MusicLoad() {}

#[forward]
#[export_name="BASS_Pause"]
pub extern "C" fn BASS_Pause() {}

#[forward]
#[export_name="BASS_PluginEnable"]
pub extern "C" fn BASS_PluginEnable() {}

#[forward]
#[export_name="BASS_PluginFree"]
pub extern "C" fn BASS_PluginFree() {}

#[forward]
#[export_name="BASS_PluginGetInfo"]
pub extern "C" fn BASS_PluginGetInfo() {}

#[forward]
#[export_name="BASS_PluginLoad"]
pub extern "C" fn BASS_PluginLoad() {}

#[forward]
#[export_name="BASS_RecordFree"]
pub extern "C" fn BASS_RecordFree() {}

#[forward]
#[export_name="BASS_RecordGetDevice"]
pub extern "C" fn BASS_RecordGetDevice() {}

#[forward]
#[export_name="BASS_RecordGetDeviceInfo"]
pub extern "C" fn BASS_RecordGetDeviceInfo() {}

#[forward]
#[export_name="BASS_RecordGetInfo"]
pub extern "C" fn BASS_RecordGetInfo() {}

#[forward]
#[export_name="BASS_RecordGetInput"]
pub extern "C" fn BASS_RecordGetInput() {}

#[forward]
#[export_name="BASS_RecordGetInputName"]
pub extern "C" fn BASS_RecordGetInputName() {}

#[forward]
#[export_name="BASS_RecordInit"]
pub extern "C" fn BASS_RecordInit() {}

#[forward]
#[export_name="BASS_RecordSetDevice"]
pub extern "C" fn BASS_RecordSetDevice() {}

#[forward]
#[export_name="BASS_RecordSetInput"]
pub extern "C" fn BASS_RecordSetInput() {}

#[forward]
#[export_name="BASS_RecordStart"]
pub extern "C" fn BASS_RecordStart() {}

#[forward]
#[export_name="BASS_SampleCreate"]
pub extern "C" fn BASS_SampleCreate() {}

#[forward]
#[export_name="BASS_SampleFree"]
pub extern "C" fn BASS_SampleFree() {}

#[forward]
#[export_name="BASS_SampleGetChannel"]
pub extern "C" fn BASS_SampleGetChannel() {}

#[forward]
#[export_name="BASS_SampleGetChannels"]
pub extern "C" fn BASS_SampleGetChannels() {}

#[forward]
#[export_name="BASS_SampleGetData"]
pub extern "C" fn BASS_SampleGetData() {}

#[forward]
#[export_name="BASS_SampleGetInfo"]
pub extern "C" fn BASS_SampleGetInfo() {}

#[forward]
#[export_name="BASS_SampleLoad"]
pub extern "C" fn BASS_SampleLoad() {}

#[forward]
#[export_name="BASS_SampleSetData"]
pub extern "C" fn BASS_SampleSetData() {}

#[forward]
#[export_name="BASS_SampleSetInfo"]
pub extern "C" fn BASS_SampleSetInfo() {}

#[forward]
#[export_name="BASS_SampleStop"]
pub extern "C" fn BASS_SampleStop() {}

#[forward]
#[export_name="BASS_Set3DFactors"]
pub extern "C" fn BASS_Set3DFactors() {}

#[forward]
#[export_name="BASS_Set3DPosition"]
pub extern "C" fn BASS_Set3DPosition() {}

#[forward]
#[export_name="BASS_SetConfig"]
pub extern "C" fn BASS_SetConfig() {}

#[forward]
#[export_name="BASS_SetConfigPtr"]
pub extern "C" fn BASS_SetConfigPtr() {}

#[forward]
#[export_name="BASS_SetDevice"]
pub extern "C" fn BASS_SetDevice() {}

#[forward]
#[export_name="BASS_SetEAXParameters"]
pub extern "C" fn BASS_SetEAXParameters() {}

#[forward]
#[export_name="BASS_SetVolume"]
pub extern "C" fn BASS_SetVolume() {}

#[forward]
#[export_name="BASS_Start"]
pub extern "C" fn BASS_Start() {}

#[forward]
#[export_name="BASS_Stop"]
pub extern "C" fn BASS_Stop() {}

#[forward]
#[export_name="BASS_StreamCreate"]
pub extern "C" fn BASS_StreamCreate() {}

#[forward]
#[export_name="BASS_StreamCreateFile"]
pub extern "C" fn BASS_StreamCreateFile() {}

#[forward]
#[export_name="BASS_StreamCreateFileUser"]
pub extern "C" fn BASS_StreamCreateFileUser() {}

#[forward]
#[export_name="BASS_StreamCreateURL"]
pub extern "C" fn BASS_StreamCreateURL() {}

#[forward]
#[export_name="BASS_StreamFree"]
pub extern "C" fn BASS_StreamFree() {}

#[forward]
#[export_name="BASS_StreamGetFilePosition"]
pub extern "C" fn BASS_StreamGetFilePosition() {}

#[forward]
#[export_name="BASS_StreamPutData"]
pub extern "C" fn BASS_StreamPutData() {}

#[forward]
#[export_name="BASS_StreamPutFileData"]
pub extern "C" fn BASS_StreamPutFileData() {}

#[forward]
#[export_name="BASS_Update"]
pub extern "C" fn BASS_Update() {}

// #[forward]
// #[export_name="_"]
// pub extern "C" fn _() {}

