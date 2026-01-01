// ## 🔷 **Problem 1: Print Name Function**

// **Category:** Functions | الدوال  
// **Difficulty:** Easy | سهل  
// **Source:** programmingadvices.com

// ### 📝 **Description | الوصف:**
// Write a program that contains a function to print a given name with a formatted message.

// اكتب برنامجًا يحتوي على دالة لطباعة اسم معطى مع رسالة منسقة.

// ### 📋 **Requirements | المتطلبات:**
// - Create a function `PrintName` that accepts a string parameter
// - The function should display the name with the message "Your Name is: "
// - Call the function from main with a hardcoded name

// - إنشاء دالة `PrintName` تقبل معامل نصي
// - يجب أن تعرض الدالة الاسم مع الرسالة "Your Name is: "
// - استدعاء الدالة من main باسم ثابت

// ### 🎯 **Expected Output | المخرجات المتوقعة:**
// ```
// Your Name is: Abdullah
// ```

// ### 📌 **Key Concepts | المفاهيم الأساسية:**
// - Functions with parameters | الدوال مع المعاملات
// - String handling | التعامل مع النصوص
// - Console output | الإخراج إلى وحدة التحكم

// ---

fn print_name(name: &str) {
  println!("Your Name is {}",name);
}

fn main() {
  print_name("Abdullah");
}