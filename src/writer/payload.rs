use super::util;
use super::util::Writable;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    pub Timestamp: String,
    pub Type: String,
    pub SubType: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NetGen {
    pub Timestamp: String,
    pub Type: String,
    pub SubType: String,
    pub SendIP: String,
    pub RecvIP: String,
    pub Protocol: String,
    pub TimeDelta: f64,
}

impl Writable for NetGen {
    fn fmt(&self) -> String {
        format!(
            "{}",
            format!(
                "{} {} {} {} {} {}",
                self.Type, self.SubType, self.SendIP, self.RecvIP, self.Protocol, self.TimeDelta,
            )
            .magenta()
        )
    }
    fn log(&self) -> Result<String, serde_json::Error> {
        util::try_serialize::<Self>(self)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StructHost {
    pub Timestamp: String,
    pub Type: String,
    pub SubType: String,
    pub OperatingSystem: String,
    pub OSType: String,
    pub Architecture: String,
    pub Name: String,
    pub NCPU: i32,
    pub MemTotal: i64,
    pub KernelVersion: String,
}

impl Writable for StructHost {
    fn fmt(&self) -> String {
        format!(
            "{}",
            format!(
                "{} {} {} {} {}",
                self.Type, self.SubType, self.OperatingSystem, self.OSType, self.Name,
            )
            .magenta()
        )
    }
    fn log(&self) -> Result<String, serde_json::Error> {
        util::try_serialize::<Self>(self)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StructNet {
    pub Timestamp: String,
    pub Type: String,
    pub SubType: String,
    pub ID: String,
    pub Name: String,
}

impl Writable for StructNet {
    fn fmt(&self) -> String {
        format!(
            "{}",
            format!(
                "{} {} {} {}",
                self.Type,
                self.SubType,
                &self.ID.as_str()[..12],
                self.Name
            )
            .magenta()
        )
    }
    fn log(&self) -> Result<String, serde_json::Error> {
        util::try_serialize::<Self>(self)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StructContNet {
    pub PrivatePort: u16,
    pub PublicPort: u16,
    pub Type: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StructCont {
    pub Timestamp: String,
    pub Type: String,
    pub SubType: String,
    pub ID: String,
    pub Name: String,
    pub Image: String,
    pub CPUPerc: String,
    pub Locale: String,
    pub Timezone: String,
    pub IPAddresses: HashMap<String, String>,
    pub Ports: HashMap<String, Vec<StructContNet>>,
}

impl Writable for StructCont {
    fn fmt(&self) -> String {
        format!(
            "{}",
            format!(
                "{} {} {} {}",
                self.Type,
                self.SubType,
                &self.ID.as_str()[..12],
                self.Name,
            )
            .magenta()
        )
    }
    fn log(&self) -> Result<String, serde_json::Error> {
        util::try_serialize::<Self>(self)
    }
}
