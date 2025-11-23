use std::{fmt, net::IpAddr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseCode {
    Successful = 0,
    Initializing = 1,
    InternalError = 2,
    InvalidRequest = 3,
    InvalidParameter = 4,
    Guarded = 5,
    TimeOut = 6,
    FirmwareUpdating = 99,
    AccessError = 100,
    OtherErrors = 101,
    WrongUserName = 102,
    WrongPassword = 103,
    AccountExpired = 104,
    AccountDisconnected = 105,
    AccountLimitReached = 106,
    ServerMaintenance = 107,
    InvalidAccount = 108,
    LicenseError = 109,
    ReadOnlyMode = 110,
    MaxStations = 111,
    AccessDenied = 112,
    NeedSpecifyPlaylist = 113,
    NeedCreatePlaylist = 114,
    SimultaneousLoginsLimit = 115,
    LinkingInProgress = 200,
    UnlinkingInProgress = 201,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ResponseCode::Successful => "Successful",
            ResponseCode::Initializing => "Initializing",
            ResponseCode::InternalError => "Internal Error",
            ResponseCode::InvalidRequest => "Invalid Request",
            ResponseCode::InvalidParameter => "Invalid Parameter",
            ResponseCode::Guarded => "Guarded (Operation not allowed)",
            ResponseCode::TimeOut => "Request Timed Out",
            ResponseCode::FirmwareUpdating => "Firmware Updating",
            ResponseCode::AccessError => "Access Error",
            ResponseCode::OtherErrors => "Other Error",
            ResponseCode::WrongUserName => "Wrong Username",
            ResponseCode::WrongPassword => "Wrong Password",
            ResponseCode::AccountExpired => "Account Expired",
            ResponseCode::AccountDisconnected => "Account Disconnected",
            ResponseCode::AccountLimitReached => "Account Limit Reached",
            ResponseCode::ServerMaintenance => "Server Under Maintenance",
            ResponseCode::InvalidAccount => "Invalid Account",
            ResponseCode::LicenseError => "License Error",
            ResponseCode::ReadOnlyMode => "Read-Only Mode",
            ResponseCode::MaxStations => "Maximum Stations Reached",
            ResponseCode::AccessDenied => "Access Denied",
            ResponseCode::NeedSpecifyPlaylist => "Playlist Must Be Specified",
            ResponseCode::NeedCreatePlaylist => "Playlist Must Be Created",
            ResponseCode::SimultaneousLoginsLimit => "Simultaneous Logins Limit Reached",
            ResponseCode::LinkingInProgress => "Linking In Progress",
            ResponseCode::UnlinkingInProgress => "Unlinking In Progress",
        };
        write!(f, "{}", description)
    }
}

impl From<u32> for ResponseCode {
    fn from(code: u32) -> Self {
        match code {
            0 => ResponseCode::Successful,
            1 => ResponseCode::Initializing,
            2 => ResponseCode::InternalError,
            3 => ResponseCode::InvalidRequest,
            4 => ResponseCode::InvalidParameter,
            5 => ResponseCode::Guarded,
            6 => ResponseCode::TimeOut,
            99 => ResponseCode::FirmwareUpdating,
            100 => ResponseCode::AccessError,
            101 => ResponseCode::OtherErrors,
            102 => ResponseCode::WrongUserName,
            103 => ResponseCode::WrongPassword,
            104 => ResponseCode::AccountExpired,
            105 => ResponseCode::AccountDisconnected,
            106 => ResponseCode::AccountLimitReached,
            107 => ResponseCode::ServerMaintenance,
            108 => ResponseCode::InvalidAccount,
            109 => ResponseCode::LicenseError,
            110 => ResponseCode::ReadOnlyMode,
            111 => ResponseCode::MaxStations,
            112 => ResponseCode::AccessDenied,
            113 => ResponseCode::NeedSpecifyPlaylist,
            114 => ResponseCode::NeedCreatePlaylist,
            115 => ResponseCode::SimultaneousLoginsLimit,
            200 => ResponseCode::LinkingInProgress,
            201 => ResponseCode::UnlinkingInProgress,
            _ => ResponseCode::OtherErrors,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YamahaDevice {
    pub ip: IpAddr,
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DeviceInfo {
    pub model_name: String,
    pub destination: String,
    pub device_id: String,
    pub system_id: String,
    pub system_version: f32,
    pub api_version: f32,
    pub netmodule_generation: u32,
    pub netmodule_version: String,
    pub netmodule_checksum: String,
    pub serial_number: String,
    pub operation_mode: String,
    pub update_error_code: String,
    pub net_module_num: u32,
    pub update_data_type: u32,
    pub analytics_info: AnalyticsInfo,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AnalyticsInfo {
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ZoneStatus {
    pub power: String,
    pub sleep: u32,
    pub volume: u32,
    pub mute: bool,
    pub max_volume: u32,
    pub input: String,
    pub input_text: String,
    pub distribution_enable: bool,
    pub sound_program: String,
    pub surr_decoder_type: String,
    pub pure_direct: bool,
    pub enhancer: bool,
    pub tone_control: ToneControl,
    pub dialogue_level: u32,
    pub dialogue_lift: u32,
    pub subwoofer_volume: u32,
    pub link_control: String,
    pub link_audio_delay: String,
    pub disable_flags: u32,
    pub contents_display: bool,
    pub actual_volume: ActualVolume,
    pub party_enable: bool,
    pub extra_bass: bool,
    pub adaptive_drc: bool,
    pub dts_dialogue_control: u32,
    pub adaptive_dsp_level: bool,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ToneControl {
    pub mode: String,
    pub bass: u32,
    pub treble: u32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ActualVolume {
    pub mode: String,
    pub value: f32,
    pub unit: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ZoneProgramList {
    pub sound_program_list: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SignalInfo {
    pub audio: AudioSignal,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AudioSignal {
    pub error: u32,
    pub format: String,
    pub fs: String,
    pub bitrate: u32,
    pub bit: String
}
