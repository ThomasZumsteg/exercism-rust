pub fn is_leap_year(year: isize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
