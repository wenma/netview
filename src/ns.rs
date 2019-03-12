
use std::ffi::CString;
use std::ffi::NulError;

use std::path::Path;
use std::process;

const IPROUTE_NETNS_ROOT: &'static str = "/var/run/netns";
const DOCKER_NETNS_ROOT: &'static str = "/var/run/docker/netns";

#[derive(Debug)]
pub enum NsError {
    IoError(String),
    Nul(NulError),
}

impl From<NulError> for NsError {
     fn from(err: NulError) -> Self {
          NsError::Nul(err)
     }
}

pub trait WithNetns {
    fn with_netns(&self);
}


pub struct Namespaced {
    fd: i32,
    src_pid: u32,
}

impl WithNetns for Namespaced {
    fn with_netns(&self) {
        unsafe {
           libc::setns(self.get_fd(), libc::CLONE_NEWNET);
        }
   }
}

impl Default for Namespaced {
    fn default() -> Self { 
        Namespaced { fd: -1, src_pid: process::id() } 
    }
}

impl Drop for Namespaced {
    fn drop(&mut self) {
       if let Err(e) = self.restore() {
            println!("failed to restore the source pid namespace, {:?}", e);
        }
    }
}

#[allow(dead_code)]
impl Namespaced {

    pub fn name<'a>(mut self, name: &'a str) -> Result<Self, NsError> {
        let path = &format!("{}/{}", IPROUTE_NETNS_ROOT, name);

        let root = if Path::new(path).exists() { IPROUTE_NETNS_ROOT } else { DOCKER_NETNS_ROOT };
        let nspath: &str = &format!("{}/{}", root, name);
        self.open(nspath)?;
        Ok(self)
    }

    pub fn pid(mut self, pid: u32) -> Result<Self, NsError> {
        let nspath: &str = &format!("/proc/{}/ns/net", pid);
        self.open(nspath)?;
        Ok(self)
    }

    pub fn fd(mut self, fd: i32) -> Result<Self, NsError> {
        self.fd = fd;
        Ok(self)
    }

    fn open(&mut self, nspath: &str) -> Result<(), NsError> {
        let path = CString::new(nspath)?;
        unsafe {
            self.fd = libc::open(path.as_ptr(), libc::O_RDONLY);
        }
        if self.fd == -1 { 
            Err(NsError::IoError(format!("failed to open {}.", nspath)))
        } else { 
            Ok(()) 
        }
    }

    fn get_fd(&self) -> i32 {
        self.fd
    }

    fn restore(&mut self) -> Result<(), NsError> {
        self.open(&format!("/proc/{}/ns/net", self.src_pid))?;
        unsafe {
            libc::setns(self.get_fd(), 0);
        }
        Ok(())
    }
}

fn walk_netns_names(root_path: &str) -> std::io::Result<Vec<String>> {
    let mut names: Vec<String> = Vec::new();

    let path = Path::new(root_path);
    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            if let Some(op) = entry.path().file_name() {
                if let Some(p) = op.to_str() {
                    names.push(String::from(p));
                }
            }    
        }
    }
    
    Ok(names)

}

pub fn ns_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    let sys_ns_names = walk_netns_names(IPROUTE_NETNS_ROOT);
    let docker_ns_names =  walk_netns_names(DOCKER_NETNS_ROOT); 

    if let Ok(mut name) = sys_ns_names {
        names.append(&mut name);
    }
    
    if let Ok(mut name) = docker_ns_names {
        names.append(&mut name);
    }

    names
}
