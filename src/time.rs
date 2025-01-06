
use chrono::DateTime;
use uts2ts::uts2ts;
pub fn get_day_month_time(epoch_secs: u64) -> String {
    let ts = uts2ts(epoch_secs as i64);
    let year = ts.year;

   

    let month = match ts.month {
        1 => "JAN",
        2 => "FEB",
        3 => "MARCH",
        4 => "APRIL",
        5 => "MAY",
        6 => "JUNE",
        7 => "JULY",
        8 => "AUG",
        9 => "SEPT",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => unreachable!(),
    };
    let f = format!("{month} {:02}, {year}  {}:{}", ts.day, ts.hour, ts.second);

    DateTime::from_timestamp(epoch_secs as i64, 0).map_or( f ,|f| format!("{}", f.format("%b %d, %Y %I:%M %p")))

}