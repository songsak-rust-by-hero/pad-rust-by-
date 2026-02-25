pub fn satang_to_baht(satang: i64) -> String {
    let sign = if satang < 0 { "-" } else { "" };
    let abs_satang = satang.unsigned_abs();

    let baht = abs_satang / 100;
    let remainder = abs_satang % 100;

    // ใช้ :02 เพื่อบังคับให้มีเลข 0 นำหน้าถ้ามีหลักเดียว (เช่น 05 สตางค์)
    format!("{}{}.{:02} บาท", sign, baht, remainder)
}
