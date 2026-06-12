#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, BytesN, symbol_short, Symbol};

// Cấu hình các khóa lưu trữ hệ thống công khai
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const AI_SERVER_KEY: Symbol = symbol_short!("AI_SERVER");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Driver {
    pub expiry_timestamp: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccidentReport {
    pub driver_address: Address,
    pub video_hash: BytesN<32>, 
    pub victim_address: Address,
    pub compensation_amount: i128,
    pub settled: bool,
}

#[contracttype]
pub enum DataKey {
    Driver(Address),
    Accident(BytesN<32>),
}

#[contract]
pub struct DecentralizedBlackBox;

#[contractimpl]
impl DecentralizedBlackBox {
    
    // Khởi tạo Admin và Server AI
    pub fn initialize(env: Env, admin: Address, ai_server: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("Hop dong da duoc khoi tao tuoc do!");
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&AI_SERVER_KEY, &ai_server);
    }

    // Tài xế mua bảo hiểm
    pub fn buy_insurance(env: Env, driver: Address) {
        driver.require_auth();

        let current_time = env.ledger().timestamp();
        let key = DataKey::Driver(driver.clone());
        
        let mut expiry = current_time + 2592000; // 30 ngày

        if env.storage().persistent().has(&key) {
            let existing_driver: Driver = env.storage().persistent().get(&key).unwrap();
            if existing_driver.expiry_timestamp > current_time {
                expiry = existing_driver.expiry_timestamp + 2592000;
            }
        }

        let new_driver = Driver {
            expiry_timestamp: expiry,
            is_active: true,
        };

        env.storage().persistent().set(&key, &new_driver);
    }

    // Server AI kích hoạt lệnh đền bù
    pub fn trigger_compensation(
        env: Env,
        incident_id: BytesN<32>,
        driver: Address,
        victim: Address,
        video_hash: BytesN<32>,
        damage_level: u32,
    ) {
        let ai_server: Address = env.storage().instance().get(&AI_SERVER_KEY).unwrap();
        ai_server.require_auth();

        let accident_key = DataKey::Accident(incident_id.clone());
        if env.storage().persistent().has(&accident_key) {
            panic!("Vu tai nan nay da duoc giai quyet!");
        }

        let driver_key = DataKey::Driver(driver.clone());
        if !env.storage().persistent().has(&driver_key) {
            panic!("Tai xe khong tham gia bao hiem!");
        }
        
        let driver_data: Driver = env.storage().persistent().get(&driver_key).unwrap();
        if !driver_data.is_active || driver_data.expiry_timestamp < env.ledger().timestamp() {
            panic!("Bao hiem cua tai xe da het han!");
        }

        let mut compensation_amount: i128 = 500; 
        if damage_level == 2 {
            compensation_amount = 1000;          
        } else if damage_level == 3 {
            compensation_amount = 1500;          
        }

        let report = AccidentReport {
            driver_address: driver,
            video_hash,
            victim_address: victim,
            compensation_amount,
            settled: true,
        };
        env.storage().persistent().set(&accident_key, &report);
    }
}