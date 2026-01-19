// ════════════════════════════════════════════════════════════════════════════
// ## 🔷 Problem 11: average_with_pass_fail.rs
// ════════════════════════════════════════════════════════════════════════════
// **Category:** Math & Conditional Logic | الرياضيات والمنطق الشرطي
// **Difficulty:** Easy | سهل
// **Source:** programmingadvices.com
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// 📝 DESCRIPTION | الوصف
// ────────────────────────────────────────────────────────────────────────────
//
// English:
// Write a program that reads three exam marks from the user, calculates their
// average, and determines whether the student has passed or failed. A student
// passes if the average is 50 or greater. The program should:
// - Read three integer marks from the user
// - Calculate the sum of the marks
// - Compute the average (as a floating-point number)
// - Check if average >= 50 (Pass) or < 50 (Fail)
// - Display the average and the pass/fail status
//
// العربية:
// اكتب برنامجًا يقرأ ثلاث علامات امتحان من المستخدم، ويحسب متوسطها، ويحدد ما إذا
// كان الطالب قد نجح أو رسب. الطالب ينجح إذا كان المتوسط 50 أو أكثر. يجب على البرنامج:
// - قراءة ثلاث علامات صحيحة من المستخدم
// - حساب مجموع العلامات
// - حساب المتوسط (كرقم عشري)
// - التحقق من المتوسط >= 50 (نجاح) أو < 50 (رسوب)
// - عرض المتوسط وحالة النجاح/الرسوب
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 💡 EXAMPLES | الأمثلة
// ────────────────────────────────────────────────────────────────────────────
//
// Example 1 (Pass):
// Input:  Mark1 = 60, Mark2 = 70, Mark3 = 80
// Output: Your Average is: 70
//         You Passed
//
// Example 2 (Fail):
// Input:  Mark1 = 30, Mark2 = 40, Mark3 = 45
// Output: Your Average is: 38.333332
//         You Failed
//
// Example 3 (Boundary - Pass):
// Input:  Mark1 = 50, Mark2 = 50, Mark3 = 50
// Output: Your Average is: 50
//         You Passed
//
// Example 4 (Boundary - Fail):
// Input:  Mark1 = 49, Mark2 = 49, Mark3 = 50
// Output: Your Average is: 49.333332
//         You Failed
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// ⚠️ CONSTRAINTS | القيود
// ────────────────────────────────────────────────────────────────────────────
//
// • Marks should be integers (0-100 recommended) | العلامات أعداد صحيحة (يُنصح 0-100)
// • Pass threshold is 50 (inclusive) | عتبة النجاح هي 50 (شاملة)
// • Average is computed as f32 for precision | المتوسط يُحسب كـ f32 للدقة
// • Input validation recommended but not required | التحقق من المدخلات مُوصى به لكن غير مطلوب
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 🔧 FUNCTION SIGNATURES | توقيعات الدوال
// ────────────────────────────────────────────────────────────────────────────
//
// enum PassFail { Pass, Fail }
// fn read_marks() -> (i32, i32, i32)
// fn sum_of_3_marks(mark1: i32, mark2: i32, mark3: i32) -> i32
// fn calculate_average(mark1: i32, mark2: i32, mark3: i32) -> f32
// fn check_average(average: f32) -> PassFail
// fn print_results(average: f32)
//
// ────────────────────────────────────────────────────────────────────────────
use std::io::{self, Write};

// ======================
//      CONSTANTS
// ======================
const PASS_THRESHOLD: f32 = 50.0;
const NUM_MARKS: usize = 3;

// ======================
//        ENUMS
// ======================
enum PassFail {
    Pass,
    Fail,
}

// ======================
//     INPUT FUNCTIONS
// ======================
fn read_mark(prompt: &str) -> i32 {
    loop {
        let input: i32 = read_number(prompt);

        if (0..=100).contains(&input) {
            return input;
        } else {
            println!("Mark must be between 0 and 100!");
        }
    }
}

fn read_marks() -> (i32, i32, i32) {
    println!("\nEnter three exam marks (0-100):");
    println!("{}", "-".repeat(30));

    let mark1 = read_mark("Mark 1: ");
    let mark2 = read_mark("Mark 2: ");
    let mark3 = read_mark("Mark 3: ");

    (mark1, mark2, mark3)
}

// ======================
//   PROCESSING FUNCTIONS
// ======================
fn sum_of_3_marks(mark1: i32, mark2: i32, mark3: i32) -> i32 {
    mark1
        .checked_add(mark2)
        .and_then(|sum| sum.checked_add(mark3))
        .expect("Arithmetic overflow in sum calculation!")
}

fn calculate_average(mark1: i32, mark2: i32, mark3: i32) -> f32 {
    sum_of_3_marks(mark1, mark2, mark3) as f32 / NUM_MARKS as f32
}

fn check_average(average: f32) -> PassFail {
    if average >= PASS_THRESHOLD {
        PassFail::Pass
    } else {
        PassFail::Fail
    }
}

// ======================
//    OUTPUT FUNCTIONS
// ======================
fn print_results(mark1: i32, mark2: i32, mark3: i32) {
    let sum = sum_of_3_marks(mark1, mark2, mark3);
    let average = calculate_average(mark1, mark2, mark3);
    let result = check_average(average);

    println!("\n{}", "=".repeat(40));
    println!("EXAM RESULTS ANALYSIS");
    println!("{}", "=".repeat(40));
    println!("Marks:     {}, {}, {}", mark1, mark2, mark3);
    println!("Sum:       {}", sum);
    println!("Average:   {:.2}", average);
    println!("Threshold: {}", PASS_THRESHOLD);
    println!("{}", "-".repeat(40));

    match result {
        PassFail::Pass => {
            println!("Status:    PASSED");
            let margin = average - PASS_THRESHOLD;
            println!("Margin:    +{:.2} points above", margin);
        }
        PassFail::Fail => {
            println!("Status:    FAILED");
            let deficit = PASS_THRESHOLD - average;
            println!("Deficit:   {:.2} points below", deficit);
        }
    }

    println!("{}", "=".repeat(40));
}

// ======================
//        MAIN
// ======================
fn main() {
    println!("AVERAGE WITH PASS/FAIL CHECKER");
    println!("{}", "-".repeat(30));

    let (m1, m2, m3) = read_marks();
    print_results(m1, m2, m3);
}

// ======================
//   UTILITY FUNCTIONS
// ======================
fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush output!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    input.trim().to_string()
}

fn read_number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = read_string(prompt);

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}
