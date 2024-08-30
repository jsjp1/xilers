pub trait NetworkInterface {
    fn connect() -> Result<(), String>;
}
