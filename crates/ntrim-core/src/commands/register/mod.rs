use log::info;
use prost::Message;
use ntrim_macros::command;
use pb::trpc::register::{ * };
use crate::pb;

struct RegisterBuilder;

#[command("trpc.msg.register_proxy.RegisterProxy.SsoInfoSync", "register", Protobuf, Register)]
impl RegisterBuilder {
    async fn generate(bot: &Arc<Bot>) -> Option<Vec<u8>> {
        let session = bot.client.session.clone();
        let mut session = session.write().await;
        info!("Generating register request for bot: {:?}", session.uid);
        let protocol = &(session.protocol);
        let device = &(session.device);

        let mut c2c_sync_info = SsoC2cSyncInfo::default();
        if c2c_sync_info.c2c_last_msg_time == 0 {
            c2c_sync_info.c2c_last_msg_time = 0;
            c2c_sync_info.last_c2c_msg_cookie = vec![];
            c2c_sync_info.c2c_msg_cookie = vec![];
        } else {
            let mut cookie = SsoC2cMsgCookie::default();
            cookie.c2c_last_msg_time = session.last_c2c_msg_time;
            c2c_sync_info.c2c_last_msg_time = session.last_c2c_msg_time;
            c2c_sync_info.last_c2c_msg_cookie = cookie.encode_to_vec();
            c2c_sync_info.c2c_msg_cookie = cookie.encode_to_vec();
        }

        let mut normal_cfg = NormalConfig::default();
        normal_cfg.int_cfg.push(NormalIntCfgEntry {
            key: 46,
            value: 0,
        });
        normal_cfg.int_cfg.push(NormalIntCfgEntry {
            key: 283,
            value: 0,
        });
        let mut register_info = RegisterInfo::default();
        register_info.guid = hex::encode(session.guid.as_slice());
        register_info.kick_pc = 0;
        register_info.build_ver = protocol.nt_build_version.to_string();
        register_info.is_first_register_proxy_online = 1;
        register_info.locale_id = 2052;
        register_info.device_info = DeviceInfo {
            dev_name: format!("{}-{}", device.device_model, device.device_name),
            dev_type: device.device_model.to_string(),
            os_ver: device.os_ver.to_string(),
            brand: if device.device_model == "Xiaomi" {
                "MIUI".to_string()
            } else {
                device.device_model.to_string()
            },
            vendor_os_name: "V140".to_string(),
        };
        register_info.set_mut = 0;
        register_info.register_vendor_type = 6;
        register_info.reg_type = 0;
        register_info.online_busi_info = OnLineBusinessInfo {
            notify_switch: 1,
            bind_uin_notify_switch: 1,
        };
        register_info.battery_status = 0;
        let req = SsoSyncInfoRequest {
            sync_flag: 735,
            req_random: rand::random::<u32>(),
            cur_active_status: 2,
            group_last_msg_time: session.last_grp_msg_time,
            c2c_sync_info,
            normal_config: normal_cfg,
            register_info,
            unknown: UnknownStructure {
                group_code: 0,
                flag2: 1
            },
            app_state: CurAppState {
                is_delay_request: 0,
                app_status: 1,
                silence_status: 0
            }
        };
        //info!("encoded: {:?}", hex::encode(req.encode_to_vec()));
        Some(req.encode_to_vec())
    }

    async fn parse(bot: &Arc<Bot>, data: Vec<u8>) -> Option<SsoSyncInfoResponse> {
        println!("recv: {}", hex::encode(data.as_slice()));
        Some(SsoSyncInfoResponse::default())
    }
}
