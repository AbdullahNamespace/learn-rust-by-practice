// ════════════════════════════════════════════════════════════════════════════
// ## 🔷 Problem 16: rectangle_area_by_side_and_diagonal.rs
// ════════════════════════════════════════════════════════════════════════════
// **Category:** Geometry / Mathematics | الهندسة / الرياضيات
// **Difficulty:** Easy | سهل
// **Source:** programmingadvices.com
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// 📝 DESCRIPTION | الوصف
// ────────────────────────────────────────────────────────────────────────────
//
// EN:
// Write a program that calculates the area of a rectangle using one side (A)
// and its diagonal (D). The program should:
// 1. Read the side length (A) from the user
// 2. Read the diagonal length (D) from the user
// 3. Calculate the area using the formula: Area = A × √(D² - A²)
// 4. Display the computed area
//
// The formula is derived from the Pythagorean theorem where the other side
// can be calculated as: B = √(D² - A²)
//
// AR:
// اكتب برنامجًا يحسب مساحة مستطيل باستخدام ضلع واحد (A) والقطر (D).
// يجب على البرنامج:
// 1. قراءة طول الضلع (A) من المستخدم
// 2. قراءة طول القطر (D) من المستخدم
// 3. حساب المساحة باستخدام الصيغة: المساحة = A × √(D² - A²)
// 4. عرض المساحة المحسوبة
//
// الصيغة مشتقة من نظرية فيثاغورس حيث يمكن حساب الضلع الآخر كـ: B = √(D² - A²)
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 💡 EXAMPLES | الأمثلة
// ────────────────────────────────────────────────────────────────────────────
//
// Example 1:
// Input:  Side A = 3.0, Diagonal D = 5.0
// Output: Rectangle Area = 12.0
//   Why:  Other side B = √(5² - 3²) = √(25 - 9) = √16 = 4.0
//         Area = 3.0 × 4.0 = 12.0
//
// Example 2:
// Input:  Side A = 5.0, Diagonal D = 13.0
// Output: Rectangle Area = 60.0
//   Why:  Other side B = √(13² - 5²) = √(169 - 25) = √144 = 12.0
//         Area = 5.0 × 12.0 = 60.0
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// ⚠️ CONSTRAINTS | القيود
// ────────────────────────────────────────────────────────────────────────────
//
// • Side A must be positive (A > 0) | يجب أن يكون الضلع A موجباً
// • Diagonal D must be greater than side A (D > A) | يجب أن يكون القطر D أكبر من الضلع A
// • Input should be valid floating-point numbers | يجب أن تكون القيم أرقاماً عشرية صحيحة
//
// ────────────────────────────────────────────────────────────────────────────

// ────────────────────────────────────────────────────────────────────────────
// 🔧 FUNCTION SIGNATURES | توقيعات الدوال
// ────────────────────────────────────────────────────────────────────────────
//
// fn read_rectangle_measurements() -> (f32, f32)
// fn calculate_rectangle_area_by_side_and_diagonal(side_a: f32, diagonal_d: f32) -> f32
// fn print_rectangle_area(area: f32)
//
// ────────────────────────────────────────────────────────────────────────────

use std::io::{self, Write};

// ======================
//     ERROR TYPE
// ======================

enum RectangleError {
    InvalidSide,
    InvalidDiagonal,
}

impl std::fmt::Display for RectangleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidSide => write!(f, "Side must be positive"),
            Self::InvalidDiagonal => write!(f, "Diagonal must be greater than side"),
        }
    }
}

// ======================
//     INPUT FUNCTIONS
// ======================

fn read_positive_float(prompt: &str) -> f32 {
    loop {
        let input = read_number::<f32>(prompt);

        if input > 0.0 {
            return input;
        } else {
            println!("❌ Error: must be a positive number\n");
        }
    }
}

fn read_rectangle_measurements() -> (f32, f32) {
    println!("Enter rectangle measurements:");
    print_line();

    let side = read_positive_float(" Side A: ");

    loop {
        let diagonal = read_positive_float("  Diagonal D: ");

        if diagonal > side {
            return (side, diagonal);
        } else {
            eprintln!("❌ Error: Diagonal must be greater than side A!\n");
        }
    }
}

// ======================
//  CALCULATION FUNCTIONS
// ======================

fn calculate_rectangle_area_by_side_and_diagonal(
    side: f32,
    diagonal: f32,
) -> Result<f32, RectangleError> {
    if side <= 0.0 {
        return Err(RectangleError::InvalidSide);
    }
    if diagonal <= side {
        return Err(RectangleError::InvalidDiagonal);
    }

    let diff = diagonal.powi(2) - side.powi(2);

    let other_side = diff.sqrt();

    Ok(side * other_side)
}

// ======================
//     OUTPUT FUNCTIONS
// ======================

fn print_rectangle_area(side: f32, diagonal: f32) {
    match calculate_rectangle_area_by_side_and_diagonal(side, diagonal) {
        Ok(area) => {
            let other_side = (diagonal.powi(2) - side.powi(2)).sqrt();

            println!("╔═══════════════════════════════════╗");
            println!("║    Rectangle Calculation          ║");
            println!("╠═══════════════════════════════════╣");
            println!("║  Side A:       {:>10.2}         ║", side);
            println!("║  Diagonal D:   {:>10.2}         ║", diagonal);
            println!("║  Other side B: {:>10.2}         ║", other_side);
            println!("╠═══════════════════════════════════╣");
            println!("║  Area:          {:>10.2}        ║", area);
            println!("╚═══════════════════════════════════╝\n");
        }
        Err(e) => {
            eprintln!("\n❌ Error: {}\n", e);
        }
    }
}

// ======================
//          MAIN
// ======================

fn main() {
    print_line();
    println!("Rectangle Area Calculator");
    print_line();

    let (side, diagonal) = read_rectangle_measurements();
    print_rectangle_area(side, diagonal);
}

// ======================
//     UTILITY FUNCTIONS
// ======================

fn read_string(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn read_number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = match read_string(prompt) {
            Ok(value) => value,
            Err(_) => {
                println!("Error: invalid input, ");
                continue;
            }
        };

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Error: invalid input, ")
            }
        }
    }
}

fn print_line() {
    println!("{}", "═".repeat(30));
}
