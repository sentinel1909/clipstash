use clipstash::data::AppDatabase;
use clipstash::web::renderer::Renderer;
use dotenv::dotenv;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "httpd")]
struct Args {
    #[arg(default_value = "sqlite:data.db")]
    connection_string: String,
    #[arg(short, long, default_value = "templates/")] // need to add equivalent of parse(from_os_str)
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let rt = tokio::runtime::Runtime::new()
        .expect("failed to spawn tokio runtime");

        let handle = rt.handle().clone();

        rt.block_on(async move {
            let renderer = Renderer::new(args.template_directory);
            let database = AppDatabase::new(&args.connection_string).await;

            let config = clipstash::RocketConfig {
                renderer,
                database,
            };

            clipstash::rocket(config)
                .launch()
                .await
                .expect("failed to launch rocket server")
        });
}