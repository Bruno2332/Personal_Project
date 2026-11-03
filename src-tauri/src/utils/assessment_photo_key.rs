use keyring::Entry;
use rand_core::{OsRng, RngCore};
use std::{fs, io, path::Path, sync::Mutex};

static KEY_LOCK: Mutex<()> = Mutex::new(());

fn key_entry() -> Result<Entry, String> {
    let account = if cfg!(debug_assertions) {
        "assessment-photos-dev"
    } else {
        "assessment-photos"
    };

    Entry::new("personal_trainer_project.photos", account)
        .map_err(|_| "Não foi possível acessar o cofre das fotos.".to_string())
}

fn decode_key(value: &str) -> Result<[u8; 32], String> {
    let mut key = [0u8; 32];

    hex::decode_to_slice(value, &mut key)
        .map_err(|_| "A chave das fotos possui formato inválido.".to_string())?;

    Ok(key)
}

pub fn load_key() -> Result<[u8; 32], String> {
    let value = key_entry()?.get_password().map_err(|error| match error {
        keyring::Error::NoEntry => "A chave das fotos não foi encontrada no cofre.".to_string(),
        _ => "Não foi possível recuperar a chave das fotos.".to_string(),
    })?;

    decode_key(&value)
}

pub fn load_or_create_key(data_dir: &Path) -> Result<[u8; 32], String> {
    let _guard = KEY_LOCK
        .lock()
        .map_err(|_| "Não foi possível acessar o controle da chave das fotos.".to_string())?;

    let entry = key_entry()?;

    match entry.get_password() {
        Ok(value) => decode_key(&value),

        Err(keyring::Error::NoEntry) => {
            // Não cria uma chave diferente se já houver fotos criptografadas.
            if has_encrypted_files(data_dir)? {
                return Err(
                    "Existem fotos criptografadas, mas a chave não foi encontrada. Recupere a chave original antes de adicionar fotos."
                        .to_string(),
                );
            }

            let mut key = [0u8; 32];

            OsRng
                .try_fill_bytes(&mut key)
                .map_err(|_| "Não foi possível gerar a chave das fotos.".to_string())?;

            let encoded = hex::encode(key);

            entry
                .set_password(&encoded)
                .map_err(|_| "Não foi possível guardar a chave das fotos no cofre.".to_string())?;

            // Só permite gravar fotos após confirmar a leitura da chave.
            let stored = entry.get_password().map_err(|_| {
                "Não foi possível confirmar a chave das fotos no cofre.".to_string()
            })?;

            if stored != encoded {
                return Err(
                    "A chave recuperada do cofre não corresponde à chave criada.".to_string(),
                );
            }

            Ok(key)
        }

        Err(_) => Err("Não foi possível recuperar a chave das fotos.".to_string()),
    }
}

fn has_encrypted_files(data_dir: &Path) -> Result<bool, String> {
    let entries = match fs::read_dir(data_dir.join("assessment_photos")) {
        Ok(entries) => entries,

        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(false);
        }

        Err(_) => {
            return Err("Não foi possível verificar os arquivos das fotos.".to_string());
        }
    };

    for entry in entries {
        let entry =
            entry.map_err(|_| "Não foi possível verificar um arquivo das fotos.".to_string())?;

        let filename = entry.file_name();
        let filename = filename.to_string_lossy();

        if filename.ends_with(".ptphoto") || filename.starts_with(".deleting-") {
            return Ok(true);
        }
    }

    Ok(false)
}
