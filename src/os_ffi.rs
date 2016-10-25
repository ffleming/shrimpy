extern {
    pub fn fork() -> i32;
    pub fn getegid() -> u32;
    pub fn geteuid() -> u32;
    pub fn getgid() -> u32;
}
