#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Cấu trúc dữ liệu lưu trữ thông tin Chứng nhận (Certificate)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    id: u64,
    recipient: String,    // Tên người nhận
    course_name: String,  // Tên khóa học/kỹ năng
    issue_date: String,   // Ngày cấp
}

// Khóa lưu trữ (Storage key) cho danh sách chứng nhận
const CERT_DATA: Symbol = symbol_short!("CERT_DAT");

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {
    // Hàm lấy danh sách tất cả chứng nhận đã cấp
    pub fn get_all_certificates(env: Env) -> Vec<Certificate> {
        return env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));
    }

    // Hàm cấp một chứng nhận mới
    pub fn issue_certificate(env: Env, recipient: String, course_name: String, issue_date: String) -> String {
        // 1. Lấy danh sách chứng nhận hiện tại từ storage
        let mut certs: Vec<Certificate> = env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Tạo một ID ngẫu nhiên và đóng gói đối tượng Certificate mới
        let cert = Certificate {
            id: env.prng().gen::<u64>(),
            recipient,
            course_name,
            issue_date,
        };
        
        // 3. Thêm chứng nhận mới vào danh sách
        certs.push_back(cert);
        
        // 4. Lưu lại vào storage của contract
        env.storage().instance().set(&CERT_DATA, &certs);
        
        return String::from_str(&env, "Cap chung nhan thanh cong!");
    }

    // Hàm thu hồi hoặc xóa chứng nhận dựa trên ID
    pub fn revoke_certificate(env: Env, id: u64) -> String {
        // 1. Lấy danh sách từ storage
        let mut certs: Vec<Certificate> = env.storage().instance().get(&CERT_DATA).unwrap_or(Vec::new(&env));

        // 2. Duyệt tìm chứng nhận có ID trùng khớp để xóa
        for i in 0..certs.len() {
            if certs.get(i).unwrap().id == id {
                certs.remove(i);

                // Cập nhật lại storage sau khi xóa
                env.storage().instance().set(&CERT_DATA, &certs);
                return String::from_str(&env, "Da thu hoi chung nhan thanh cong.");
            }
        }

        return String::from_str(&env, "Khong tim thay chung nhan phu hop.");
    }
}

mod test;