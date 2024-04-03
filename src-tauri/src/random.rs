use passwords::PasswordGenerator;
use uuid::Uuid;

#[tauri::command]
pub async fn gen_passwords(
    count: usize,
    length: usize,
    numbers: bool,
    loweralpha: bool,
    upperalpha: bool,
    symbols: bool,
    strict: bool,
) -> Result<Vec<String>, String> {
    let pg = PasswordGenerator {
        length,
        numbers,
        lowercase_letters: loweralpha,
        uppercase_letters: upperalpha,
        symbols,
        strict,
        spaces: false,
        exclude_similar_characters: false,
    };

    pg.generate(count).map_err(|e| e.into())
}

#[tauri::command]
pub fn gen_uuid_v4() -> String {
    Uuid::new_v4().into()
}

#[tauri::command]
pub fn gen_uuid_v7() -> String {
    Uuid::now_v7().into()
}
