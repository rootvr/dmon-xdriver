use super::payload;
use super::serdes::SerDes;
use super::util;
use crate::cli::Args;
use crossbeam_channel::Receiver;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Debug)]
pub struct Dispatcher<'a> {
    stime: &'a Instant,
    serdes_netgen: SerDes,
    serdes_structhost: SerDes,
    serdes_structnet: SerDes,
    serdes_structcont: SerDes,
}

impl<'a> Dispatcher<'a> {
    pub fn new(cli_args: &Arc<Args>, wgen_handle: &Arc<Mutex<Child>>, stime: &'a Instant) -> Self {
        Self {
            stime,
            serdes_netgen: SerDes::new(&cli_args, &wgen_handle, "net.gen".to_string()),
            serdes_structhost: SerDes::new(&cli_args, &wgen_handle, "struct.host".to_string()),
            serdes_structnet: SerDes::new(&cli_args, &wgen_handle, "struct.net".to_string()),
            serdes_structcont: SerDes::new(&cli_args, &wgen_handle, "struct.cont".to_string()),
        }
    }
    pub fn dispatch(
        &mut self,
        rx: Receiver<String>,
        cli_args: &Arc<Args>,
        wgen_handle: &Arc<Mutex<Child>>,
        xtime: f64,
    ) {
        for data in rx {
            match wgen_handle.lock().unwrap().try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => {}
                Err(_) => {
                    kill!(wgen_handle, "error during child process wait");
                }
            }
            let etime = self.stime.elapsed();
            match util::try_deserialize::<payload::Base>(&data) {
                Ok(json_data) => {
                    if json_data.Type == "network" {
                        if cli_args.filter != "structure" {
                            if json_data.SubType == "general" {
                                self.serdes_netgen.log::<payload::NetGen>(
                                    &data,
                                    &cli_args,
                                    &wgen_handle,
                                    &etime,
                                    xtime,
                                );
                            }
                        }
                    } else if json_data.Type == "structure" {
                        if cli_args.filter != "network" {
                            if json_data.SubType == "host" {
                                self.serdes_structhost.log::<payload::StructHost>(
                                    &data,
                                    &cli_args,
                                    &wgen_handle,
                                    &etime,
                                    xtime,
                                );
                            } else if json_data.SubType == "container" {
                                self.serdes_structcont.log::<payload::StructCont>(
                                    &data,
                                    &cli_args,
                                    &wgen_handle,
                                    &etime,
                                    xtime,
                                );
                            } else if json_data.SubType == "network" {
                                self.serdes_structnet.log::<payload::StructNet>(
                                    &data,
                                    &cli_args,
                                    &wgen_handle,
                                    &etime,
                                    xtime,
                                );
                            }
                        }
                    }
                }
                Err(_) => {
                    kill!(wgen_handle, "unable to parse data {}", data);
                }
            }
        }
        self.close(wgen_handle);
    }
    fn close(&mut self, wgen_handle: &Arc<Mutex<Child>>) {
        self.serdes_netgen.close(wgen_handle);
        self.serdes_structhost.close(wgen_handle);
        self.serdes_structnet.close(wgen_handle);
        self.serdes_structcont.close(wgen_handle);
    }
}
