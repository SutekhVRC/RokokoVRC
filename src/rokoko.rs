
pub mod models {

    pub mod types {
        use serde::Deserialize;


        #[derive(Debug, Deserialize)]
        pub struct SpacialVector3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Deserialize)]
        pub struct SpacialVector4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Deserialize)]
        pub struct SpacialTransform {
            pub position: SpacialVector3,
            pub rotation: SpacialVector4,
        }
    }

    pub mod rokoko {

        use serde::Deserialize;
        use serde_json::Value;
        use super::types::{SpacialTransform, SpacialVector3};

        #[derive(Debug, Deserialize)]
        pub struct RokokoActorMeta {
            #[serde(rename(deserialize = "hasGloves"))]
            pub has_gloves: bool,
            
            #[serde(rename(deserialize = "hasLeftGlove"))]
            pub has_left_glove: bool,

            #[serde(rename(deserialize = "hasBody"))]
            pub has_body: bool,

            #[serde(rename(deserialize = "hasFace"))]
            pub has_face: bool,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoActorDimensions {

            #[serde(rename(deserialize = "totalHeight"))]
            pub total_height: f32,

            #[serde(rename(deserialize = "hipHeight"))]
            pub hip_height: f32,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoActorbodyHip {
            pub position: SpacialVector3,
            pub rotation: SpacialVector3,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoActorBody {
            
            pub hip: SpacialTransform,
            
            pub chest: SpacialTransform,
            
            #[serde(rename(deserialize = "leftLowerArm"))]
            pub left_lower_arm: SpacialTransform,

            #[serde(rename(deserialize = "rightLowerArm"))]
            pub right_lower_arm: SpacialTransform,
            
            #[serde(rename(deserialize = "rightLeg"))]
            pub right_leg: SpacialTransform,

            #[serde(rename(deserialize = "rightFoot"))]
            pub right_foot: SpacialTransform,

            #[serde(rename(deserialize = "leftLeg"))]
            pub left_leg: SpacialTransform,

            #[serde(rename(deserialize = "leftFoot"))]
            pub left_foot: SpacialTransform,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoActorFace {
    
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoActor {
            pub name: String,
            //pub color: [u8; 3],
            pub meta: RokokoActorMeta,
            pub dimensions: RokokoActorDimensions,
            pub body: RokokoActorBody,
            //pub face: RokokoActorFace,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoScene {
            pub timestamp: f64,
            pub actors: Vec<RokokoActor>,
            #[serde(skip)]
            pub props: Vec<Value>,
        }
    
        #[derive(Debug, Deserialize)]
        pub struct RokokoUpdate {
            #[serde(skip)]
            pub version: String,
            #[serde(skip)]
            pub fps: f32,
            pub scene: RokokoScene,
        }
    }
}