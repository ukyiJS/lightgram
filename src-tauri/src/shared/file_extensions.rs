// 파일 확장자 목록을 관리하는 모듈

// 일반 이미지 파일 확장자 (JPG 포함)
pub const NORMAL_IMAGE_EXTENSIONS: [&str; 10] = [
    "jpg", "jpeg", "png", "gif", "tiff", "tif", "bmp", "webp", "heic", "heif",
];

// RAW 파일 확장자
pub const RAW_EXTENSIONS: [&str; 24] = [
    "cr2", "nef", "arw", "dng", "orf", "pef", "raf", "rw2", "srw", "x3f", "raw", "rwl", "mrw",
    "3fr", "kdc", "dcr", "nrw", "mef", "srf", "sr2", "crw", "erf", "fff", "mdc",
];

// 모든 이미지 파일 확장자 (일반 + RAW)
pub const IMAGE_EXTENSIONS: [&str; 34] = [
    // 일반 이미지 파일 형식
    "jpg", "jpeg", "png", "gif", "tiff", "tif", "bmp", "webp", "heic", "heif",
    // RAW 파일 형식
    "cr2", "nef", "arw", "dng", "orf", "pef", "raf", "rw2", "srw", "x3f", "raw", "rwl", "mrw",
    "3fr", "kdc", "dcr", "nrw", "mef", "srf", "sr2", "crw", "erf", "fff", "mdc",
];

// RAW 파일과 함께 이동해야 하는 메타데이터 확장자
pub const RAW_METADATA_EXTENSIONS: [&str; 1] = ["xmp"];

// 이미지 확장자 확인 함수 (일반 + RAW)
pub fn is_supported_image(ext: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// 일반 이미지 확장자 확인 함수 (JPG 포함)
pub fn is_normal_image(ext: &str) -> bool {
    NORMAL_IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// JPG 확장자 확인 함수
pub fn is_jpg_file(ext: &str) -> bool {
    let lower_ext = ext.to_lowercase();
    lower_ext == "jpg" || lower_ext == "jpeg"
}

// RAW 확장자 확인 함수
pub fn is_raw_file(ext: &str) -> bool {
    RAW_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// RAW 메타데이터 확장자 확인 함수
pub fn is_raw_metadata_file(ext: &str) -> bool {
    RAW_METADATA_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}
