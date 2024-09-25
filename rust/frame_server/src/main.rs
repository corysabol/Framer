use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use btleplug::api::{Central, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter};

use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use uuid::Uuid;

// BLE Stuff

async fn frame_write(
    device: btleplug::platform::Peripheral,
    send: Characteristic,
    input: String,
) -> Result<(), Box<dyn std::error::Error>> {
    device
        .write(
            &send,
            input.as_bytes(),
            btleplug::api::WriteType::WithResponse,
        )
        .await?;

    Ok(())
}

async fn frame_pair() -> Result<(), Box<dyn std::error::Error>> {
    // BLE SETUP ========
    let RX_CHARACTERISTIC_UUID: Uuid =
        Uuid::from_str("7A230003-5475-A6A4-654C-8431F6AD49C4").expect("Unable to parse RX UUID");
    let TX_CHARACTERISTIC_UUID: Uuid =
        Uuid::from_str("7A230002-5475-A6A4-654C-8431F6AD49C4").expect("Unable to parse TX UUID");
    let SERVICE_UUID: Uuid = Uuid::from_str("7A230001-5475-A6A4-654C-8431F6AD49C4")
        .expect("Unable to parse SERVICE UUID");

    // Create a manager to interact with the local Bluetooth adapter
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters
        .into_iter()
        .nth(0)
        .expect("Unable to find adapters.");
    adapter.start_scan(ScanFilter::default()).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let devices = adapter.peripherals().await?;
    let mut device = None;
    for d in devices {
        if d.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Frame"))
        {
            device = Some(d);
        }
    }

    // Connect to the device
    let device = device.expect("Failed to find a matching device");
    device.connect().await?;

    // Wait to allow for user to accept OS pairing request
    tokio::time::sleep(Duration::from_secs(5)).await;

    device.discover_services().await?;

    let chars = device.characteristics();
    let recv = chars
        .iter()
        .find(|c| c.uuid == RX_CHARACTERISTIC_UUID)
        .expect("Could not find RX characteristic");
    let send = chars
        .iter()
        .find(|c| c.uuid == TX_CHARACTERISTIC_UUID)
        .expect("Could not find TX characteristic");

    // Set up channels for communication between threads
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn a task to handle BLE notifications
    let device_clone = device.clone();
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        if let Ok(mut notifications) = device_clone.notifications().await {
            while let Some(data) = notifications.next().await {
                let response =
                    String::from_utf8(data.value).unwrap_or_else(|_| "Invalid UTF-8".to_string());
                let _ = tx_clone.send(response).await;
            }
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:45789").await?;
    println!("Frame BLE server listening on 127.0.0.1:45789");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let n = socket.read(&mut buf).await.unwrap();
            let cmd = std::str::from_utf8(&buf[..n]).unwrap();

            match cmd {
                "connect" => {
                    // Frame pairing logic here
                    // Expects the name of the device to pair to
                    // Defaults to looking for a BLE peripheral starting with "Frame"
                    println!("Connecting to Frame device...");
                    frame_pair().await.unwrap();
                    socket.write_all(b"Connected").await.unwrap();
                }
                "exec_lua" => {
                    println!("Executing lua code on Frame");
                    // Check if connected to device etc
                    // Exec code with execution id for pulling out any results later
                }
                "notifications" => {
                    // Pull back n notification / allow for polling of notifications from Frame
                    // These are buffered in memory until something asks for them
                    println!("Getting notification from Frame");
                }
                _ => socket.write_all(b"Unknown command").await.unwrap(),
            }
        });
    }
}
