import { invoke } from '@tauri-apps/api/core';

export interface FileInfo {
  name: string;
  path: string;
  is_file: boolean;
  extension: string | null;
  size: number;
}

/**
 * PhotoManager - 백엔드 Rust 함수를 호출하는 클래스
 * 다양한 사진 관리 및 정리 기능을 제공합니다.
 */
export class PhotoManager {
  /**
   * 디렉토리 내 모든 파일 목록을 가져옵니다.
   * @param directory 디렉토리 경로
   */
  listDirectoryFiles = async (directory: string): Promise<FileInfo[]> =>
    invoke('list_directory_files', { params: { directory } });

  /**
   * 디렉토리 내 일반 이미지(JPG 포함)와 RAW 파일 목록을 분리하여 가져옵니다.
   * @param directory 디렉토리 경로
   */
  listNormalAndRawFiles = async (directory: string): Promise<[FileInfo[], FileInfo[]]> =>
    invoke('list_normal_and_raw_files', { params: { directory } });

  /**
   * 파일을 타입별로 정리합니다.
   * 일반 파일은 확장자별 디렉토리로, RAW 파일과 XMP 파일은 'raw' 디렉토리로 이동합니다.
   * @param directory 정리할 디렉토리 경로
   */
  organizeFilesByType = async (directory: string) =>
    invoke('organize_files_by_type', { params: { directory } });

  /**
   * 사진을 메타데이터 타입별로 그룹화합니다.
   * @param directory 대상 디렉토리 경로
   * @param metadataType 그룹화 기준이 될 메타데이터 타입
   */
  groupByMetadata = async (directory: string, metadataType: MetadataType) => {
    const commandMap: Record<MetadataType, string> = {
      lens: 'group_by_lens',
      date: 'group_by_date',
      iso: 'group_by_iso',
      aperture: 'group_by_aperture',
      shutterSpeed: 'group_by_shutter_speed',
      focalLength: 'group_by_focal_length',
      cameraModel: 'group_by_camera_model',
      timeOfDay: 'group_by_time_of_day',
      gpsLocation: 'group_by_gps_location',
    };

    const command = commandMap[metadataType];

    return invoke<void>(command, { directory });
  };
}

/**
 * 그룹화에 사용될 메타데이터 타입
 */
export type MetadataType =
  'aperture' | 'cameraModel' | 'date' | 'focalLength' | 'gpsLocation' | 'iso' | 'lens' | 'shutterSpeed' | 'timeOfDay';
