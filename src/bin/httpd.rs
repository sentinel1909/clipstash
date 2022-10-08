use clap::Parser;
use clipstash::data::AppDatabase;
use clipstash::domain::maintenance::Maintenance;
use clipstash::web::{hitcounter::HitCounter, renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "httpd")]
struct Args {
    #[arg(default_value = "sqlite:data.db")]
    connection_string: String,
    #[arg(short, long, default_value = "templates/")]
    // need to add equivalent of parse(from_os_str)
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();
    let renderer = Renderer::new(args.template_directory.clone());
    let database = rt.block_on(async move { AppDatabase::new(&args.connection_string).await });

    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());
    let maintenance = Maintenance::spawn(database.get_pool().clone(), handle.clone());

    let config = clipstash::RocketConfig {
        renderer,
        database,
        hit_counter,
        maintenance,
    };

    rt.block_on(async move {
        clipstash::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")
    });
}
