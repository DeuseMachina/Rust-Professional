use chrono::{NaiveDate, Datelike};

pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let birth_date = NaiveDate::parse_from_str(time, "%Y-%m").expect("Invalid date format");
    let birth_year = birth_date.year();
    let birth_month = birth_date.month();

    // 默认法定退休年龄
    let mut retirement_age = 60; // 默认法定退休年龄60岁
    let mut delay_months = 0;

    // 确定退休年龄和延迟退休策略
    if tp.contains("女职工") {
        if tp.contains("原法定退休年龄55周岁女职工") {
            retirement_age = 55;
        } else {
            retirement_age = 60;
        }
    } else if tp.contains("男职工") {
        retirement_age = 60;
    }

    // 根据中央最新的延迟退休政策计算延迟退休的月数
    let current_year = 2025;  // 假设当前年份是2025年
    let years_since_birth = current_year - birth_year;

    // 对于每个人，不同的延迟退休政策会影响退休年龄
    if years_since_birth >= 50 {
        delay_months = (retirement_age * 12 + retirement_age as i32) - years_since_birth as i32;
    }

    // 计算实际退休时间
    let retirement_year = current_year + delay_months / 12;
    let retirement_month = (birth_month as i32 + delay_months % 12) as u32;

    // 计算退休年龄
    let retirement_age_f64 = retirement_age as f64 + (retirement_month as f64 - 1.0) / 12.0;

    // 格式化输出
    let retirement_month_str = if retirement_month == 0 { 12 } else { retirement_month };
    let retirement_year_month = format!("{:04}-{:02}", retirement_year, retirement_month_str);

    // 返回格式：退休时间，退休年龄，延迟退休月份
    format!("{}, {:.2}, {}", retirement_year_month, retirement_age_f64, delay_months)
}