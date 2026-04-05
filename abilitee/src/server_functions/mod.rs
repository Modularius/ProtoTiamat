mod session;

use chrono::SubsecRound;
use libertee::Timestamp;

pub use session::{PerformLogin, PerformLogout, Register, get_session_from_identity};

pub fn format_datetime(datetime: &Timestamp) -> String {
    let date = datetime.date_naive();
    let time = datetime.time().trunc_subsecs(0);
    format!("{date}, {time}")
}
