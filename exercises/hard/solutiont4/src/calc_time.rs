pub fn time_info(time: &str) -> String {
    let parts : Vec<&str> = time.split('-').collect();
    let year: i32 = parts[0].parse().expect("Invalid time");
    let month: i32 = parts[1].parse().expect("Invalid time");
    let day: i32 = parts[2].parse().expect("Invalid time");
    let week_num = week_of_year(year,month,day);
    let weekday = day_of_week(year,month,day);
    let days = day_of_year(year,month,day);
    let left_days = days_left(year,month,day);
    let days_cny = days_to_cny(year,month,day);
    let days_a = days_to_a(year,month,day);

    format!(
        "{},{},{},{},{},{}",
        week_num, weekday, days, left_days, days_cny, days_a
    )
}

fn is_leap_year(year:i32) -> bool{
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn day_of_month(year:i32, month:i32) -> i32{
    let not_leap = [0,31,28,31,30,31,30,31,31,30,31,30,31];
    let leap = [0,31,29,31,30,31,30,31,31,30,31,30,31];
    if is_leap_year(year){
        leap[month as usize]
    }else {
        not_leap[month as usize]
    }
}

fn day_of_year(year:i32, month:i32, day:i32) -> i32{
    let mut days_sum = 0;
    for i in 1..month{
        days_sum += day_of_month(year,i);
    }
    days_sum + day
}

fn days_left(year:i32, month:i32, day:i32) -> i32{
    if is_leap_year(year){
        366 - day_of_year(year,month,day)
    }else {
        365 - day_of_year(year,month,day)
    }
}

fn day_of_week(year:i32, month:i32, day:i32) -> i32{
    let mut y = year;
    let mut m = month;
    if m <= 2 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;

    // 泽勒公式的计算过程
    let f = day + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j) - 1;
    let index = (f+7) % 7;

    if index == 0 {7} else {index}
}

fn days_diff(year1:i32, month1:i32, day1:i32, year2:i32, month2:i32, day2:i32) -> i32{
    let mut days1 = 0i32;
    let mut days2 = 0i32;
    for y in 2000..year1{
        days1 += if is_leap_year(y) {366} else {365};
    }
    for y in 2000..year2{
        days2 += if is_leap_year(y) {366} else {365};
    }
    days1 += day_of_year(year1,month1,day1);
    days2 += day_of_year(year2,month2,day2);

    (days1 - days2).abs()
}

//rely on iso 8601
fn first_week_begin(year : i32) -> (i32, i32, i32){
    let mut first_thursday = 1;
    for i in 1..=7{
        let day = day_of_week(year, 1, i);
        if day == 4{
            first_thursday = i;
            break;
        }
    }
    if first_thursday < 4{
        (year-1, 12,28 + first_thursday)
    }else {
        (year,1,first_thursday+3)
    }
}

fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let (y, m, d) = first_week_begin(year);
    let mut diff = days_diff(year, month, day, y, m, d);
    diff /= 7;

    if month == 12 && day >= 29 && day <= 31 {
        let (y2,m2,d2) = first_week_begin(year + 1);
        if m2 == 12 && d2 <= day {
            return 1;
        }
    }

    1 + diff
}

fn days_to_cny(year: i32, month: i32, day: i32) -> i32 {
    let current_day = day_of_year(year, month, day);
    let new_year_day = day_of_year(year, 1, 29);
    let next_new_year = day_of_year(year+1,2,17);
    if current_day < new_year_day {
        new_year_day - current_day
    } else {
        365 - current_day + next_new_year
    }
}

fn days_to_a(year: i32, month: i32, day: i32) -> i32 {
    let new_year_day_open = day_of_year(year, 1, 2);
    let spring_year_day_open = day_of_year(year, 2, 5);
    let qingming_day_open = day_of_year(year, 4, 7);
    let labor_day_open = day_of_year(year, 5, 6);
    let zongzi_day_open = day_of_year(year, 6, 3);
    let autumn_day_open = day_of_year(year, 10, 9);
    let next_new_year_day_open = day_of_year(year+1, 1, 1);

    let current_day = day_of_year(year,month,day);

    // 处理节假日
    if month == 1 && day == 1 {
        return new_year_day_open - current_day - 1;
    }
    if (month == 1 && 28 <= day && day <= 31) || (month == 2 && 1 <= day && day <= 4) {
        return spring_year_day_open - current_day - 1;
    }
    if month == 4 && 4 <= day && day <= 6 {
        return qingming_day_open - current_day - 1;
    }
    if month == 5 && 1 <= day && day <= 5 {
        return labor_day_open - current_day - 1;
    }
    if (month == 5 && day == 31) || (month == 6 && 1 <= day && day <= 2) {
        return zongzi_day_open - current_day- 1;
    }

    if month == 10 && 1 <= day && day <= 8 {
        return autumn_day_open - current_day - 1;
    }

    if month == 12 && day == 31 {
        let days = if is_leap_year(year) { 366 } else { 365 };
        return next_new_year_day_open - current_day + days;
    }

    let weekday = day_of_week(year, month, day);
    match weekday {
        7 => 0,
        5 => 2, 
        6 => 1, 
        _ => 0, 
    }
}