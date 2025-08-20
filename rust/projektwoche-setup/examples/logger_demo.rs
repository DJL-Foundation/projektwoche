use projektwoche_setup::logger::{
    ConsoleOutput, LevelFilter, LogCollector, LogLevel, Logger, LoggerSystem,
};
use std::thread;
use std::time::Duration;

fn main() {
    // Create the logger system
    let (logger_system, mut collector) = LoggerSystem::new();

    // Add console output with colors
    collector.add_output(Box::new(ConsoleOutput::new(true)));

    // Add level filter (Info and above)
    collector.add_filter(Box::new(LevelFilter::new(LogLevel::Info)));

    // Start the log collector in a background thread
    let collector_handle = thread::spawn(move || {
        collector.run();
    });

    // Create loggers for different "components"
    let main_logger = logger_system.create_logger("main", "main-thread".to_string());
    let worker1_logger = logger_system.create_logger("worker", "worker-1".to_string());
    let worker2_logger = logger_system.create_logger("worker", "worker-2".to_string());

    main_logger.info("Starting multi-threaded operation");

    // Simulate multiple threads doing work
    let handles = vec![
        {
            let logger = worker1_logger;
            thread::spawn(move || {
                logger.info("Starting installation of Node.js");
                thread::sleep(Duration::from_millis(500));
                logger.info("Downloading Node.js installer...");
                thread::sleep(Duration::from_millis(300));
                logger.info("Running installer...");
                thread::sleep(Duration::from_millis(800));
                logger.info("Node.js installation completed successfully");
            })
        },
        {
            let logger = worker2_logger;
            thread::spawn(move || {
                logger.info("Starting installation of Git");
                thread::sleep(Duration::from_millis(200));
                logger.warn("Git is already installed, checking version...");
                thread::sleep(Duration::from_millis(400));
                logger.info("Git version check passed");
                thread::sleep(Duration::from_millis(100));
                logger.info("Configuring Git defaults...");
                thread::sleep(Duration::from_millis(600));
                logger.info("Git configuration completed successfully");
            })
        },
    ];

    main_logger.info("Waiting for all installations to complete...");

    // Wait for all worker threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    main_logger.info("All installations completed successfully!");

    // Drop the logger system to signal the collector to stop
    drop(logger_system);

    // Wait for the collector to finish
    collector_handle.join().unwrap();

    println!("Demo completed!");
}