#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short, token};

const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const TEAM_WALLET: Symbol = symbol_short!("TEAM_W");
const IS_STUDENT: Symbol = symbol_short!("STUDENT"); // Key để kiểm tra nhãn sinh viên

#[contract]
pub struct SoundPulsePro;

#[contractimpl]
impl SoundPulsePro {
    pub fn init(env: Env, admin: Address, team: Address) {
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&TEAM_WALLET, &team);
    }

    /// Admin hoặc Hệ thống xác nhận một ví là Sinh viên
    pub fn verify_student(env: Env, user: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();
        env.storage().persistent().set(&user, &true); // Lưu nhãn sinh viên vào bộ nhớ
    }

    /// Mua Premium với cơ chế phí linh hoạt
    pub fn buy_premium(env: Env, user: Address, token_address: Address) {
        user.require_auth();
        let client = token::Client::new(&env, &token_address);
        let team_v: Address = env.storage().instance().get(&TEAM_WALLET).unwrap();
        let admin_v: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();

        let total_price: i128 = 200; // 2 Token
        
        // Kiểm tra xem có phải sinh viên không
        let is_student: bool = env.storage().persistent().get(&user).unwrap_or(false);

        let (team_fee, net_amount) = if is_student {
            // Gói sinh viên: Thu phí 5%, trả lại 95% giá trị vào hệ thống
            let fee = (total_price * 5) / 100;
            (fee, total_price - fee)
        } else {
            // Gói thường: Thu phí 15%, trả lại 85%
            let fee = (total_price * 15) / 100;
            (fee, total_price - fee)
        };

        // Thực hiện chuyển tiền
        client.transfer(&user, &team_v, &team_fee);
        client.transfer(&user, &admin_v, &net_amount);

        // Gia hạn 30 ngày
        let expiry = env.ledger().timestamp() + 2_592_000;
        env.storage().persistent().set(&user, &expiry);
    }

    /// Kiểm tra trạng thái và hạn dùng (Để hiển thị trên App)
    pub fn get_status(env: Env, user: Address) -> (bool, u64) {
        let is_student = env.storage().persistent().get(&user).unwrap_or(false);
        let expiry = env.storage().persistent().get(&user).unwrap_or(0);
        (is_student, expiry)
    }
}