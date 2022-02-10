pub mod logger{
    use std::io::Write;

    #[allow(dead_code)]
    pub enum LoggingLevel {
        All,
        Fatal,
        Error,
        Warn,
        Debug,
        Info,
        Off,
    }
    #[allow(dead_code)]
    pub fn LoggingLevel_as_int(level: &LoggingLevel) -> i16
    {
        match level {
            LoggingLevel::Off => 0,
            LoggingLevel::Fatal => 10,
            LoggingLevel::Error => 20,
            LoggingLevel::Warn => 30,
            LoggingLevel::Info => 40,
            LoggingLevel::Debug => 50,
            LoggingLevel::All => 60,
            _ => 40
        }
    }
    #[allow(dead_code)]
    pub struct Logger {
        path: String,
        logging_level: LoggingLevel
    }
    #[allow(dead_code)]
    pub struct Event {
        level: LoggingLevel,
        description: String
    }
    #[allow(dead_code)]
    impl Logger {
        pub fn new(level: LoggingLevel) -> Self{
    
            return Logger { path: String::from("./logs"), logging_level: level }
        }
        pub fn log(&self, event : &Event){
            println!("{} <= {}", LoggingLevel_as_int(&event.level), LoggingLevel_as_int(&self.logging_level));

            if LoggingLevel_as_int(&event.level) <= LoggingLevel_as_int(&self.logging_level) ||
             LoggingLevel_as_int(&self.logging_level) == LoggingLevel_as_int(&LoggingLevel::All){
                let path: &str = &format!("{}/{}", self.path, chrono::Local::now().format("%y-%m-%d"))[..];
                let local_time = chrono::Local::now().format("%y-%m-%d %H:%M:%S");
                let message = format!("{}: {} \n",local_time,&event.description);
                self.write(&path, &message);
                print!("{message}");
        
            }
       }
       fn write(&self, path: &str, data : &String)
       {
            let f = std::fs::OpenOptions::new().append(true).create(true).open(path).expect("Unable to open file");
            let mut f = std::io::BufWriter::new(f);
            f.write(data.as_bytes()).expect("Unable to write data");
       }
    }
    impl Event {
        #[allow(dead_code)]
        pub fn new(_level: LoggingLevel, _description : String) -> Self{
            return Event { level: _level, description: _description }
        }
    }
}