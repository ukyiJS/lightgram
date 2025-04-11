import { invoke } from '@tauri-apps/api/core';

/**
 * PhotoManager - 백엔드 Rust 함수를 호출하는 클래스
 * 다양한 사진 관리 및 정리 기능을 제공합니다.
 */
export class PhotoManager {
  /**
   * 파일을 확장자별로 정리합니다.
   * @param directory 정리할 디렉토리 경로
   */
  organizeByExtension = async (directory: string) => invoke('organize_files_by_extension', {
    params: { directory },
  });

  /**
   * RAW 이미지 파일을 JPG로 변환합니다.
   * @param inputDir 입력 디렉토리 경로
   * @param outputDir 출력 디렉토리 경로 (선택사항)
   */
  convertRawToJpg = async (inputDir: string, outputDir?: string) => invoke('convert_raw_files_to_jpg', {
    params: {
      inputDir,
      outputDir,
    },
  });

  /**
   * JPG 파일과 일치하는 RAW 파일을 찾습니다.
   * @param jpgDir JPG 파일이 있는 디렉토리 경로
   * @param rawDir RAW 파일이 있는 디렉토리 경로
   */
  getMatchedRawFiles = async (jpgDir: string, rawDir: string) => invoke('get_matched_raw_files', {
    params: {
      jpgDir,
      rawDir,
    },
  });

  /**
   * 사진을 렌즈 종류별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByLens = async (directory: string) => invoke('group_photos_by_lens', {
    params: { directory },
  });

  /**
   * 사진을 촬영 날짜별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByDate = async (directory: string) => invoke('group_photos_by_date', {
    params: { directory },
  });

  /**
   * 사진을 ISO 값별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByIso = async (directory: string) => invoke('group_photos_by_iso', {
    params: { directory },
  });

  /**
   * 사진을 조리개 값별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByAperture = async (directory: string) => invoke('group_photos_by_aperture', {
    params: { directory },
  });

  /**
   * 사진을 셔터 스피드별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByShutterSpeed = async (directory: string) => invoke('group_photos_by_shutter_speed', {
    params: { directory },
  });

  /**
   * 사진을, 초점 거리별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByFocalLength = async (directory: string) => invoke('group_photos_by_focal_length', {
    params: { directory },
  });

  /**
   * 사진을 카메라 모델별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByCameraModel = async (directory: string) => invoke('group_photos_by_camera_model', {
    params: { directory },
  });

  /**
   * 사진을 카메라 제조사별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByCameraMake = async (directory: string) => invoke('group_photos_by_camera_make', {
    params: { directory },
  });

  /**
   * 사진을 촬영 시간대별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByTimeOfDay = async (directory: string) => invoke('group_photos_by_time_of_day', {
    params: { directory },
  });

  /**
   * 사진을 GPS 위치 정보별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   */
  groupPhotosByGpsLocation = async (directory: string) => invoke('group_photos_by_gps_location', {
    params: { directory },
  });
}
