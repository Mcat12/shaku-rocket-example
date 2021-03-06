#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use shaku::ContainerBuilder;
use shaku_rocket::Inject;

use crate::autofac::{ConsoleOutput, IDateWriter, TodayWriter};

mod autofac;

#[get("/")]
fn index(writer: Inject<dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

fn main() {
    let mut builder = ContainerBuilder::new();

    builder
        .register_type::<ConsoleOutput>()
        .with_named_parameter("prefix", "PREFIX > ".to_string())
        .with_typed_parameter::<usize>(117 as usize);
    builder
        .register_type::<TodayWriter>()
        .with_typed_parameter::<String>("June 19".to_string());
    let container = builder.build().unwrap();

    rocket::ignite()
        .manage(container)
        .mount("/", routes![index])
        .launch();
}
