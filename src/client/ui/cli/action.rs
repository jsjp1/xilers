#[derive(Debug)]
pub enum ActionNum {
    DeviceList = 0,
    FileSystem,
    FileTransfer,
    Exit,
    Undefined,
}

impl ActionNum {
    pub fn iter() -> std::slice::Iter<'static, ActionNum> {
        static ACTIONS: [ActionNum; 4] = [
            ActionNum::DeviceList,
            ActionNum::FileSystem,
            ActionNum::FileTransfer,
            ActionNum::Exit,
        ];
        ACTIONS.iter()
    }
}

impl std::fmt::Display for ActionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ActionNum::DeviceList => write!(f, "DeviceList"),
            ActionNum::FileSystem => write!(f, "FileSystem"),
            ActionNum::FileTransfer => write!(f, "FileTransfer"),
            ActionNum::Exit => write!(f, "Exit"),
            ActionNum::Undefined => write!(f, "Undefined"),
        }
    }
}

impl TryFrom<i32> for ActionNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(ActionNum::DeviceList),
            1 => Ok(ActionNum::FileSystem),
            2 => Ok(ActionNum::FileTransfer),
            3 => Ok(ActionNum::Exit),
            _ => Ok(ActionNum::Undefined),
        }
    }
}
