pub enum Message {
    UploadMapInformation{
        id: String,
        url: String,
        overwrite: bool
    },
    DisplayMap {
        id: String,
    },
}

unsafe impl Send for Message { }
