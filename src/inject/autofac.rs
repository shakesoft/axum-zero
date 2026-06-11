use std::sync::Arc;
use tracing::info;

shaku::module! {
    pub AutoFacModule {
        components = [ConsoleOutput, TodayWriter],
        providers = [HelloWorldImpl]
    }
}

pub trait IOutput: shaku::Interface {
    fn write(&self, content: String);
}

#[derive(shaku::Component)]
#[shaku(interface = IOutput)]
pub struct ConsoleOutput;

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        info!("{}", content);
    }
}

pub trait IDateWriter: shaku::Interface {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(shaku::Component)]
#[shaku(interface = IDateWriter)]
pub struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
    today: String,
    year: usize,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        self.output.write(self.get_date());
    }

    fn get_date(&self) -> String {
        format!("Today is {}, {}", self.today, self.year)
    }
}

pub trait HelloWorld: shaku::Interface {
    fn greet(&self) -> String;
}

#[derive(shaku::Provider)]
#[shaku(interface = HelloWorld)]
struct HelloWorldImpl;

impl HelloWorld for HelloWorldImpl {
    fn greet(&self) -> String {
        "Hello, world!".to_owned()
    }
}

pub trait A: Send + Sync {
    fn test(&self) -> String;
}

#[dill::component]
#[dill::interface(dyn A)]
pub struct AImpl {
    b: Arc<dyn B>,
}

impl A for AImpl {
    fn test(&self) -> String {
        format!("aimpl::{}", self.b.test())
    }
}

pub trait B: Send + Sync {
    fn test(&self) -> String;
}

#[dill::component]
#[dill::interface(dyn B)]
pub struct BImpl;

impl B for BImpl {
    fn test(&self) -> String {
        "bimpl".to_owned()
    }
}