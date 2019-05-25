use std::os::raw::c_char;
use std::ffi::CString;

extern {
    fn c_parse(sql: *const c_char) -> *mut c_char;
}

pub fn parse(sql: &str) -> String {
    let c_sql = CString::new(sql).expect("Invalid SQL");
    unsafe {
        let c_ast_raw = c_parse(c_sql.as_ptr());
        let c_ast = CString::from_raw(c_ast_raw);
        c_ast.to_str()
            .expect("Could not convert parse result to owned String")
            .to_owned()
    }
}
