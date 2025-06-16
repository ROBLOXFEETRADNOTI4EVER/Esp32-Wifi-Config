#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::system::software_reset;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use esp_wifi::EspWifiController;
use wifi_ap as lib;

fn trigger_reset() -> ! {
    software_reset();
}
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");
//     let mut counter = ReloadCounter::new();
// let reload_times = counter.get_and_increment();
// defmt::info!("Device has been reloaded {} times", reload_times.await);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    // let _init = esp_wifi::init(
    //     timer1.timer0,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    // )
    // .unwrap();

    let rng = Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = &*lib::mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone(), peripherals.RADIO_CLK,).unwrap()
    );
    let mut buffer = [0u8; 32];

    let stack = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) => {
            if buffer.iter().all(|&b| b == 0xFF) {
                info!("No credentials - using AP mode stack");
                // Use your AP mode WiFi function
                lib::wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await.unwrap()
            } else {
                info!("Credentials found - using client mode stack");  
                // Use your client mode WiFi function
                lib::http_wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await
            }
        }
        Err(_) => {
            info!("Error reading flash - defaulting to AP mode");
            lib::wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await.unwrap()
        }
    };

    // bollean to spawn the good one if check from memory if the ssid and pass is pressent then do that
    // later on add a reset button to turn the bollean back if pressed more then 10 sec( the button)
    let web_app = lib::web::WebApp::default();
    let mut buffer = [0u8; 32];
    let mut host_wifi: bool = true;

    const STORAGE_OFFSET : u32 = 0x110000;

    let read_memory = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) =>{
            if buffer.iter().all(|&b| b == 0xFF) {
                info!(" LOG LOG LOG  Theres nothing in the wifi so i stop");
                let mut host_wifi: bool = true;

                let counter = lib::kiss::get_and_increment();
                spawner.must_spawn(counter);
                for id in 0..lib::web::WEB_TASK_POOL_SIZE {
                    spawner.must_spawn(lib::web::web_task(
                        id,
                        stack,
                        web_app.router,
                        web_app.config,
                    ));
                }
                info!("Web server started...");
                return ;
            }else{ 
                info!("Data found => : {:?}", defmt::Debug2Format(&buffer));
   
                let data_end = buffer.iter()
                    .position(|&x| x == 0)
                    .unwrap_or(buffer.len());
                
                let existing_data = core::str::from_utf8(&buffer[..data_end])
                    .unwrap_or("Invalid UTF-8");
                info!("  Existing credentials: {}", existing_data);
                let web_app = lib::http_web::WebApp::default();
                for id in 0..lib::http_web::WEB_TASK_POOL_SIZE {
                    spawner.must_spawn(lib::http_web::web_task(
                        id,
                        stack,
                        web_app.router,
                        web_app.config,
                    ));
                }
                info!("Web server started...");
            }
        }
        Err(_) =>{
            info!(" LOG LOG LOG  There was an error idk what");
        }
    };



}