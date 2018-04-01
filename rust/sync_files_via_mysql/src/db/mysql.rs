use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use super::Credentials;
use mysqlclient_sys::*;

unsafe fn cstr_to_string(s: *const c_char) -> String {
    CStr::from_ptr(s).to_string_lossy().into_owned()
}

#[derive(Debug, Hash)]
pub struct MySql {
    pub mysql: *mut MYSQL,
}

impl Drop for MySql {
    fn drop(&mut self) {
        unsafe {
            mysql_close(self.mysql);
        }
    }
}

use self::cstr_to_string as cs;

impl MySql {
    pub fn client_version() -> String {
        unsafe { cs(mysql_get_client_info()) }
    }

    pub fn new(p: &Credentials<CString>) -> Result<Self, String> {
        unsafe {
            let mysql = mysql_init(ptr::null_mut());
            if mysql.is_null() {
                return Err(cs(mysql_error(mysql)));
            }

            let connect_ptr = {
                let unix_socket = ptr::null();
                let client_flag = 0;
                mysql_real_connect(mysql, 
                    p.host.as_ptr(), 
                    p.user.as_ptr(), 
                    p.password.as_ptr(),
                    p.database.as_ptr(),
                    p.port,
                    unix_socket, client_flag
                )
            };
            if connect_ptr.is_null() {
                return Err(cs(mysql_error(mysql)));
            }
            /*
            if 0 != mysql_query(con, b"CREATE DATABASE testdb\0".as_ptr() as *const _) {
                eprintln!("MySQL error: {}", cs(mysql_error(con)));
            }
            */
            Ok(Self { mysql })
        }
    }
}

