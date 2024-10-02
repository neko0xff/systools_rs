use std::fs;

/*讀取路徑檔案 */
pub fn return_pathdata(path:&str) -> String {
    let data = fs::read_to_string(path)
        .unwrap_or_else(|_| "Unknown".to_string());
    let output = format!("{}",data.trim());

    output
}