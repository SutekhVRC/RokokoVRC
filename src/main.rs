
use std::net::{
    Ipv4Addr,
    SocketAddrV4,
    UdpSocket,
};

use rokoko::models::{types::SpacialTransform, rokoko::RokokoUpdate};
use rosc::{OscMessage, encoder, OscPacket, OscType};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::time::{Instant, Duration};

mod rokoko;

fn init_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 14044))
}

fn parse_packet(packet: Vec<u8>) -> Option<RokokoUpdate> {
    
    let json_raw = String::from_utf8(packet).unwrap();
    let json_raw = json_raw.trim_end_matches(char::from(0));
    
    //println!("PACKET ------------\n{}\n-------------------", json_raw);

    let json_packet: rokoko::models::rokoko::RokokoUpdate = match serde_json::from_str(json_raw) {
        Ok(jr) => jr,
        Err(_) => return None,
    };

    /*
    println!("---------------------\n[ FPS: {} ]", json_packet.fps);
    println!("[ Timestamp: {} ]", json_packet.scene.timestamp);
    println!("[ Actor: {} ]", json_packet.scene.actors[0].name);
    println!("[ Actor Height: {} ]", json_packet.scene.actors[0].dimensions.totalHeight);
    let hip = json_packet.scene.actors[0].body.get("hip").unwrap();
    println!("[ Actor Hip ]:\np: {:?}\nr: {:?}\n---------------------", hip.position, hip.rotation);
*/
    /*
    println!("TRACKING DATA -----------------\nFPS: {}\nActor Name: {}\nHeight: {}\nHead: {:?}\n----------------------------",
        json_packet.get("fps").unwrap(),
        json_packet.get("scene").unwrap().get("actors").unwrap()[0].get("name").unwrap(),
        json_packet.get("scene").unwrap().get("actors").unwrap()[0].get("dimensions").unwrap().get("totalHeight").unwrap(),
        json_packet.get("scene").unwrap().get("actors").unwrap()[0].get("body").unwrap().get("head").unwrap()
    );*/

    Some(json_packet)
}

fn send_tracking(sock: &UdpSocket, s_transforms: &SpacialTransform, scale_factor: f32, joint_id: u8) {

    let p_x = s_transforms.position.x;// * scale_factor;
    let p_y = s_transforms.position.y;// * scale_factor;
    let p_z = s_transforms.position.z;// * scale_factor;
    let r_x = s_transforms.rotation.x;
    let r_y = s_transforms.rotation.y;
    let r_z = s_transforms.rotation.z;

    /*
    println!("Updating {}..\nPosition: x: {}, y: {}, z: {}\nRotation: x: {}, y: {}, z: {}",
        joint_id,
        p_x,p_y,p_z,
        r_x,r_y,r_z,
    );
    */

    let osc_msg_pos = encoder::encode(&OscPacket::Message(OscMessage {
        addr: format!("/tracking/trackers/{}/position", joint_id),
        args: vec![OscType::Float(p_x), OscType::Float(p_y), OscType::Float(p_z)]
    })).unwrap();

    let osc_msg_rot = encoder::encode(&OscPacket::Message(OscMessage {
        addr: format!("/tracking/trackers/{}/rotation", joint_id),
        args: vec![OscType::Float(r_x), OscType::Float(r_y), OscType::Float(r_z)]
    })).unwrap();
    sock.send_to(&osc_msg_pos, "127.0.0.1:9000");
    sock.send_to(&osc_msg_rot, "127.0.0.1:9000");
}

/*
fn send_head_ref(sock: &UdpSocket, height: f32, scale_factor: f32) {
    

    println!("Setting head ref: x: {}, y: {}, z: {}",
        0.,
        (height as f32),
        0.,
    );

    let osc_msg_head_pos = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/tracking/trackers/head/position".to_string(),
        args: vec![OscType::Float(0.), OscType::Float(height as f32), OscType::Float(0.)]
    })).unwrap();

    let osc_msg_head_rot = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/tracking/trackers/head/rotation".to_string(),
        args: vec![OscType::Float(0.), OscType::Float(0.), OscType::Float(0.)]
    })).unwrap();

    sock.send_to(&osc_msg_head_pos, "127.0.0.1:9000");
    sock.send_to(&osc_msg_head_rot, "127.0.0.1:9000");
}
*/

fn listen(sock: &UdpSocket) {
    let mut height_set: bool = false;
    let scale_factor: f32 = 1.;
    println!("[+] Scale Factor: {}", scale_factor);
    
    loop {
        // Reset every second
        //let now = Instant::now();
        let mut inc = 0;
        loop {
            

            let mut buf = [0u8; 20000];
            sock.recv(&mut buf);
            let now = Instant::now();
            let packet = buf.to_vec();
            let rokoko_update = match parse_packet(packet) {
                Some(ru) => ru,
                None => continue,
            };

            //println!("Gloves: {}", rokoko_update.scene.actors[0].meta.has_gloves);

            send_tracking(sock, &rokoko_update.scene.actors[0].body.hip, scale_factor, 1);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.chest, scale_factor, 2);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.left_lower_arm, scale_factor, 3);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.right_lower_arm, scale_factor, 4);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.right_leg, scale_factor, 5);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.left_leg, scale_factor, 6);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.left_foot, scale_factor, 7);
            send_tracking(sock, &rokoko_update.scene.actors[0].body.right_foot, scale_factor, 8);

            println!("[ Recv -> Send Time ]: {} micro seconds", now.elapsed().as_micros());
            /*
            if !height_set {
                println!("[ Clip FPS ]: {}", rokoko_update.fps);
                let head_transforms_ref = rokoko_update.scene.actors[0].dimensions.totalHeight;
                send_head_ref(sock, head_transforms_ref, scale_factor);
                height_set = true;
            }
            
            inc += 1;
            if now.elapsed().as_secs() == 1 {
                println!("Updates per second: {}", inc);
                
                break;
            }
            */
        }
    }
}


fn main() {
    listen(&init_socket().unwrap());
}