use std::fs;

use crate::utils::assessment_photo_files;

#[test]
fn encrypted_photo_file_round_trips_and_removes_from_app_directory() {
    let directory = tempfile::tempdir().unwrap();
    let key = [0x5a; 32];
    let image = b"\xFF\xD8\xFFtest-jpeg-content";

    let relative_path =
        assessment_photo_files::encrypt_photo_bytes(directory.path(), image, &key).unwrap();

    let encrypted_path = directory.path().join(&relative_path);
    let encrypted_bytes = fs::read(&encrypted_path).unwrap();
    assert_ne!(encrypted_bytes, image);
    assert_eq!(
        assessment_photo_files::decrypt_photo(directory.path(), &relative_path, &key).unwrap(),
        image,
    );

    assert!(
        assessment_photo_files::decrypt_photo(directory.path(), &relative_path, &[0x33; 32],)
            .is_err()
    );

    let (original, staged) =
        assessment_photo_files::stage_deletion(directory.path(), &relative_path)
            .unwrap()
            .unwrap();
    assert!(!original.exists());
    assert!(staged.exists());

    assessment_photo_files::undo_deletion(&original, &staged).unwrap();
    assert!(original.exists());
    assessment_photo_files::remove_photo(directory.path(), &relative_path).unwrap();
    assert!(!original.exists());
}
