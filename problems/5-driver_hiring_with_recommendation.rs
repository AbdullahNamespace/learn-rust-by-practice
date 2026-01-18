// ════════════════════════════════════════════════════════════════════════════
// ## 🔷 Problem 5: driver_hiring_with_recommendation.rs
// ════════════════════════════════════════════════════════════════════════════
// **Category:** Conditional Logic | المنطق الشرطي
// **Difficulty:** Easy | سهل
// **Source:** programmingadvices.com
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// 📝 DESCRIPTION | الوصف
// ────────────────────────────────────────────────────────────────────────────
//
// English:
// Write a program that reads candidate information (age, driving license status,
// and recommendation status) and determines if they are hired or rejected for
// a driver position. A candidate is hired if:
// - They are older than 21 AND have a driving license, OR
// - They have a recommendation (regardless of age or license)
// The program includes robust input validation with error handling and accepts
// multiple input formats for boolean values (1/0, yes/no, y/n).
//
// العربية:
// اكتب برنامجًا يقرأ معلومات المرشح (العمر، حالة رخصة القيادة، وحالة التوصية)
// ويحدد ما إذا كان مقبولًا أو مرفوضًا لوظيفة سائق. يتم قبول المرشح إذا:
// - كان عمره أكبر من 21 سنة ولديه رخصة قيادة، أو
// - لديه توصية (بغض النظر عن العمر أو الرخصة)
// يتضمن البرنامج التحقق القوي من المدخلات مع معالجة الأخطاء ويقبل
// تنسيقات متعددة للقيم المنطقية (1/0، نعم/لا، y/n).
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 💡 EXAMPLES | الأمثلة
// ────────────────────────────────────────────────────────────────────────────
//
// Example 1:
// Input:  age = 25, has_driving_license = 1, has_recommendation = 0
// Output: Hired
//
// Example 2:
// Input:  age = 18, has_driving_license = 0, has_recommendation = 1
// Output: Hired
//
// Example 3 (Edge Case):
// Input:  age = 20, has_driving_license = 1, has_recommendation = 0
// Output: Rejected
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// ⚠️ CONSTRAINTS | القيود
// ────────────────────────────────────────────────────────────────────────────
//
// • Age must be between 1 and 120 | يجب أن يكون العمر بين 1 و 120
// • Boolean inputs accept: 1/0, yes/no, y/n (case insensitive) | المدخلات المنطقية تقبل: 1/0، نعم/لا، y/n
// • Invalid inputs trigger re-prompt until valid input is provided | المدخلات غير الصحيحة تطلب الإدخال مجددًا
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 🔧 FUNCTION SIGNATURES | توقيعات الدوال
// ────────────────────────────────────────────────────────────────────────────
//
// struct CandidateInfo { age: u8, has_driving_license: bool, has_recommendation: bool }
// fn read_info() -> CandidateInfo
// fn is_accepted(info: &CandidateInfo) -> bool
// fn print_result(info: &CandidateInfo)
//
// ────────────────────────────────────────────────────────────────────────────

use std::io::{self, Write};
use std::str::FromStr;

fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    input.trim().to_string()
}

fn read_boolean(prompt: &str) -> bool {
    loop {
        let input = read_string(&format!("{} [y/n]: ", prompt));

        match input.to_lowercase().as_str() {
            "1" | "y" | "yes" | "true" | "t" => return true,
            "0" | "n" | "no" | "false" | "f" => return false,
            _ => {
                println!("Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}

fn read_number<T: FromStr>(prompt: &str) -> T {
    loop {
        let input = read_string(prompt);

        match input.parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

struct CandidateInfo {
    age: u8,
    has_driving_license: bool,
    has_recommendation: bool,
}

fn read_age() -> u8 {
    loop {
        let age: i32 = read_number("Please enter Candidate's age (1-120): ");

        if !(1..=120).contains(&age) {
            println!("Age must be between 1 and 120");
            continue;
        }

        match u8::try_from(age) {
            Ok(age) => return age,
            Err(_) => println!("Invalid age value age must be in range (1..120)"),
        }
    }
}

fn read_info() -> CandidateInfo {
    let age: u8 = read_age();
    let has_driving_license: bool = read_boolean("Does candidate have driving license");
    let has_recommendation: bool = read_boolean("Does candidate have recommendation");

    CandidateInfo {
        age: age,
        has_driving_license: has_driving_license,
        has_recommendation: has_recommendation,
    }
}

fn is_accepted(info: &CandidateInfo) -> bool {
    info.has_recommendation || (info.age > 21 && info.has_driving_license)
}

fn print_result(info: &CandidateInfo) {
    if is_accepted(&info) {
        println!("Hired");
    } else {
        println!("Rejected");
    }
}

fn main() {
    print_result(&read_info());
}
