use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use rand_core::{OsRng, RngCore};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Component, Path, PathBuf},
};
use uuid::Uuid;

const PHOTO_DIRECTORY: &str = "assessment_photos";

// Identifica o formato e sua versão.
const PHOTO_HEADER: &[u8; 8] = b"PTPHOTO1";
const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;

// Limite por foto: 32 MiB.
const MAX_PHOTO_SIZE: usize = 32 * 1024 * 1024;

pub async fn run_file_task<T, F>(task: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|_| "A operação com o arquivo foi interrompida.".to_string())?
}

pub fn encrypt_photo(
    data_dir: &Path,
    source_path: &Path,
    key: &[u8; 32],
) -> Result<String, String> {
    if !source_path.is_absolute() {
        return Err("Selecione o caminho completo da foto.".to_string());
    }

    let extension = source_path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| "A foto não possui uma extensão válida.".to_string())?;

    if !matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "webp") {
        return Err("Selecione uma imagem JPG, JPEG, PNG ou WEBP.".to_string());
    }

    let source = File::open(source_path)
        .map_err(|_| "Não foi possível abrir a foto selecionada.".to_string())?;

    let metadata = source
        .metadata()
        .map_err(|_| "Não foi possível verificar a foto.".to_string())?;

    if !metadata.is_file() || metadata.len() == 0 {
        return Err("Selecione um arquivo de imagem não vazio.".to_string());
    }

    if metadata.len() > MAX_PHOTO_SIZE as u64 {
        return Err("A foto ultrapassa o limite de 32 MiB.".to_string());
    }

    let mut bytes = Vec::new();

    source
        .take(MAX_PHOTO_SIZE as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Não foi possível ler a foto selecionada.".to_string())?;

    if bytes.len() > MAX_PHOTO_SIZE {
        return Err("A foto ultrapassa o limite de 32 MiB.".to_string());
    }

    if !has_supported_image_signature(&bytes) {
        return Err("O arquivo não possui uma assinatura de JPG, PNG ou WEBP.".to_string());
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| "A chave de criptografia das fotos é inválida.".to_string())?;

    let mut nonce_bytes = [0u8; NONCE_SIZE];

    OsRng
        .try_fill_bytes(&mut nonce_bytes)
        .map_err(|_| "Não foi possível gerar os dados aleatórios da criptografia.".to_string())?;

    let nonce =
        Nonce::try_from(&nonce_bytes[..]).map_err(|_| "Tamanho de nonce inválido.".to_string())?;

    let encrypted = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: &bytes,
                aad: PHOTO_HEADER,
            },
        )
        .map_err(|_| "Não foi possível criptografar a foto.".to_string())?;

    let directory = photo_directory(data_dir)?;
    let filename = format!("{}.ptphoto", Uuid::new_v4());
    let destination = directory.join(&filename);

    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
        .map_err(|_| "Não foi possível criar o arquivo criptografado.".to_string())?;

    let written: io::Result<()> = (|| {
        output.write_all(PHOTO_HEADER)?;
        output.write_all(&nonce_bytes)?;
        output.write_all(&encrypted)?;
        output.sync_all()
    })();

    drop(output);

    if written.is_err() {
        let cleanup = fs::remove_file(&destination);

        return Err(if cleanup.is_err() {
            "Não foi possível gravar a foto criptografada nem remover o arquivo incompleto."
                .to_string()
        } else {
            "Não foi possível gravar a foto criptografada.".to_string()
        });
    }

    Ok(format!("{PHOTO_DIRECTORY}/{filename}"))
}

pub fn decrypt_photo(
    data_dir: &Path,
    relative_path: &str,
    key: &[u8; 32],
) -> Result<Vec<u8>, String> {
    let path = managed_photo_path(data_dir, relative_path)?;

    // Impede a leitura por um link simbólico colocado no lugar da foto.
    let metadata = fs::symlink_metadata(&path)
        .map_err(|_| "Não foi possível localizar o arquivo da foto.".to_string())?;

    if !metadata.file_type().is_file() {
        return Err("O caminho não corresponde a um arquivo de foto válido.".to_string());
    }

    let max_encrypted_size = PHOTO_HEADER.len() + NONCE_SIZE + MAX_PHOTO_SIZE + TAG_SIZE;

    let source = File::open(&path)
        .map_err(|_| "Não foi possível abrir a foto criptografada.".to_string())?;

    let mut encrypted_file = Vec::new();

    source
        .take(max_encrypted_size as u64 + 1)
        .read_to_end(&mut encrypted_file)
        .map_err(|_| "Não foi possível ler a foto criptografada.".to_string())?;

    if encrypted_file.len() > max_encrypted_size {
        return Err("O arquivo da foto ultrapassa o limite permitido.".to_string());
    }

    let ciphertext_start = PHOTO_HEADER.len() + NONCE_SIZE;

    if encrypted_file.len() <= ciphertext_start + TAG_SIZE
        || !encrypted_file.starts_with(PHOTO_HEADER)
    {
        return Err("O arquivo não possui o formato de foto criptografada esperado.".to_string());
    }

    let nonce_bytes = &encrypted_file[PHOTO_HEADER.len()..ciphertext_start];

    let ciphertext = &encrypted_file[ciphertext_start..];

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| "A chave de criptografia das fotos é inválida.".to_string())?;

    let nonce =
        Nonce::try_from(nonce_bytes).map_err(|_| "Tamanho de nonce inválido.".to_string())?;

    let decrypted = cipher
        .decrypt(
            &nonce,
            Payload {
                msg: ciphertext,
                aad: PHOTO_HEADER,
            },
        )
        .map_err(|_| {
            "Não foi possível descriptografar a foto: chave incorreta ou arquivo alterado."
                .to_string()
        })?;

    if !has_supported_image_signature(&decrypted) {
        return Err(
            "O conteúdo descriptografado não possui um formato de imagem suportado.".to_string(),
        );
    }

    Ok(decrypted)
}

fn has_supported_image_signature(bytes: &[u8]) -> bool {
    let jpeg = bytes.starts_with(&[0xFF, 0xD8, 0xFF]);
    let png = bytes.starts_with(b"\x89PNG\r\n\x1a\n");

    let webp = bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP";

    jpeg || png || webp
}

pub fn remove_photo(data_dir: &Path, relative_path: &str) -> Result<(), String> {
    let path = managed_photo_path(data_dir, relative_path)?;

    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err("Não foi possível remover o arquivo da foto.".to_string()),
    }
}

pub fn stage_deletion(
    data_dir: &Path,
    relative_path: &str,
) -> Result<Option<(PathBuf, PathBuf)>, String> {
    let original = managed_photo_path(data_dir, relative_path)?;

    let temporary = original.with_file_name(format!(".deleting-{}", Uuid::new_v4(),));

    match fs::rename(&original, &temporary) {
        Ok(()) => Ok(Some((original, temporary))),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("Não foi possível preparar a exclusão da foto.".to_string()),
    }
}

pub fn undo_deletion(original: &Path, temporary: &Path) -> Result<(), String> {
    fs::rename(temporary, original).map_err(|_| {
        "O banco falhou e não foi possível recolocar a foto no caminho original.".to_string()
    })
}

pub fn finish_deletion(temporary: &Path) -> Result<(), String> {
    match fs::remove_file(temporary) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(
            "O registro foi excluído, mas não foi possível remover o arquivo temporário da foto."
                .to_string(),
        ),
    }
}

fn photo_directory(data_dir: &Path) -> Result<PathBuf, String> {
    let root = data_dir
        .canonicalize()
        .map_err(|_| "A pasta do aplicativo não está disponível.".to_string())?;

    let directory = root.join(PHOTO_DIRECTORY);

    fs::create_dir_all(&directory)
        .map_err(|_| "Não foi possível criar a pasta das fotos.".to_string())?;

    let directory = directory
        .canonicalize()
        .map_err(|_| "Não foi possível acessar a pasta das fotos.".to_string())?;

    if !directory.starts_with(&root) {
        return Err("A pasta das fotos está fora da pasta do aplicativo.".to_string());
    }

    Ok(directory)
}

fn managed_photo_path(data_dir: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let mut components = Path::new(relative_path).components();

    let directory = components.next();
    let filename = components.next();

    let valid_directory = matches!(
        directory,
        Some(Component::Normal(value))
            if value == std::ffi::OsStr::new(PHOTO_DIRECTORY)
    );

    let Some(Component::Normal(filename)) = filename else {
        return Err("O caminho da foto é inválido.".to_string());
    };

    if !valid_directory || components.next().is_some() {
        return Err("O caminho da foto é inválido.".to_string());
    }

    Ok(photo_directory(data_dir)?.join(filename))
}

pub fn encrypt_photo_bytes(
    data_dir: &Path,
    bytes: &[u8],
    key: &[u8; 32],
) -> Result<String, String> {
    if bytes.is_empty() || bytes.len() > MAX_PHOTO_SIZE {
        return Err("A foto está vazia ou ultrapassa o limite de 32 MiB.".to_string());
    }

    if !has_supported_image_signature(bytes) {
        return Err("A foto não possui um formato de imagem suportado.".to_string());
    }

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| "A chave das fotos é inválida.".to_string())?;

    let mut nonce_bytes = [0u8; NONCE_SIZE];

    OsRng
        .try_fill_bytes(&mut nonce_bytes)
        .map_err(|_| "Não foi possível gerar os dados da criptografia.".to_string())?;

    let nonce =
        Nonce::try_from(&nonce_bytes[..]).map_err(|_| "Tamanho de nonce inválido.".to_string())?;

    let encrypted = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: bytes,
                aad: PHOTO_HEADER,
            },
        )
        .map_err(|_| "Não foi possível criptografar a foto.".to_string())?;

    let directory = photo_directory(data_dir)?;
    let filename = format!("{}.ptphoto", Uuid::new_v4());
    let destination = directory.join(&filename);

    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
        .map_err(|_| "Não foi possível criar a foto criptografada.".to_string())?;

    let written: io::Result<()> = (|| {
        output.write_all(PHOTO_HEADER)?;
        output.write_all(&nonce_bytes)?;
        output.write_all(&encrypted)?;
        output.sync_all()
    })();

    drop(output);

    if written.is_err() {
        return Err(match fs::remove_file(&destination) {
            Ok(()) => "Não foi possível gravar a foto criptografada.".to_string(),
            Err(_) => {
                "Não foi possível gravar a foto nem remover o arquivo incompleto.".to_string()
            }
        });
    }

    #[cfg(unix)]
    File::open(&directory)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "Não foi possível confirmar a pasta das fotos.".to_string())?;

    Ok(format!("{PHOTO_DIRECTORY}/{filename}"))
}
