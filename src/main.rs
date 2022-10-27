mod os_release;

fn main() {
    let os_release = os_release::OsRelease::new().unwrap();
    println!("{:?}", os_release);
    let os_release = os_release::OsRelease::from_file("/etc/os-release").unwrap();
    println!("{:?}", os_release);
}
