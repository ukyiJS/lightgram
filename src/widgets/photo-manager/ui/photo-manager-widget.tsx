import { open } from '@tauri-apps/plugin-dialog';
import { useEffect, useState } from 'react';

import type { FileInfo, MetadataType } from '@/shared/lib/photo-manager';
import { PhotoManager } from '@/shared/lib/photo-manager';
import { Button } from '@/shared/ui/button';
import {
  Card, CardContent, CardDescription, CardHeader, CardTitle,
} from '@/shared/ui/card';
import { Input } from '@/shared/ui/input';
import {
  Tabs, TabsContent, TabsList, TabsTrigger,
} from '@/shared/ui/tabs';

export const PhotoManagerWidget = () => {
  const [directory, setDirectory] = useState('');
  const [processingStatus, setProcessingStatus] = useState('');
  const [jpgFiles, setJpgFiles] = useState<FileInfo[]>([]);
  const [rawFiles, setRawFiles] = useState<FileInfo[]>([]);
  const [directorySelected, setDirectorySelected] = useState(false);

  const photoManager = new PhotoManager();

  // 디렉토리가 선택되면 파일 목록을 로드
  useEffect(() => {
    if (directory) {
      loadFileList();
    }
  }, [directory]);

  const loadFileList = async () => {
    try {
      setProcessingStatus('파일 목록 로딩 중...');

      const [normalImages, rawFiles] = await photoManager.listNormalAndRawFiles(directory);

      setJpgFiles(normalImages);
      setRawFiles(rawFiles);
      setDirectorySelected(true);
      setProcessingStatus('');
    } catch (error) {
      setProcessingStatus(`파일 목록 로딩 오류: ${error}`);
    }
  };

  const selectDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '폴더 선택',
      });

      if (selected && typeof selected === 'string') {
        setDirectory(selected);
      }
    } catch (error) {
      setProcessingStatus(`폴더 선택 오류: ${error}`);
    }
  };

  const handleOrganizeByFileType = async () => {
    try {
      setProcessingStatus('파일 정리 중...');
      await photoManager.organizeFilesByType(directory);
      setProcessingStatus('파일 정리 완료!');
      // 파일 정리 후 목록 새로고침
      await loadFileList();
    } catch (error) {
      setProcessingStatus(`오류 발생: ${error}`);
    }
  };

  const handleGroupPhotos = async (metadataType: MetadataType, label: string) => {
    try {
      setProcessingStatus(`사진을 ${label}(으)로 그룹화 중...`);
      await photoManager.groupByMetadata(directory, metadataType);
      setProcessingStatus(`사진을 ${label}(으)로 그룹화 완료!`);
      // 그룹화 후 파일 목록 새로고침
      await loadFileList();
    } catch (error) {
      setProcessingStatus(`오류 발생: ${error}`);
    }
  };

  // 초기 폴더 선택 화면
  if (!directorySelected) {
    return (
      <div className="container mx-auto py-6">
        <h1 className="text-3xl font-bold mb-6">사진 관리 도구</h1>
        <Card>
          <CardHeader>
            <CardTitle>시작하기</CardTitle>
            <CardDescription>작업할 폴더를 선택해주세요</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex gap-2 items-center">
              <Input
                className="flex-1"
                placeholder="폴더 경로를 입력하세요"
                value={directory}
                readOnly
              />
              <Button onClick={selectDirectory}>
                폴더 선택
              </Button>
            </div>
            {processingStatus && (
              <div className="mt-4 p-2 bg-[hsl(var(--muted))] rounded-md">
                <p className="text-center">{processingStatus}</p>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">사진 관리 도구</h1>
        <div className="flex items-center gap-2">
          <p className="text-sm text-muted-foreground">{directory}</p>
          <Button size="sm" variant="outline" onClick={selectDirectory}>
            폴더 변경
          </Button>
        </div>
      </div>
      <div className="mb-6 grid grid-cols-2 gap-4">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-xl">
              일반 이미지 파일 (
              {jpgFiles.length}
              개)
            </CardTitle>
          </CardHeader>
          <CardContent className="max-h-[200px] overflow-y-auto">
            {jpgFiles.length > 0 ? (
              <ul className="space-y-1">
                {jpgFiles.map(file => (
                  <li key={file.path} className="text-sm truncate">
                    {file.name}
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-muted-foreground text-center py-4">일반 이미지 파일이 없습니다</p>
            )}
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-xl">
              RAW 파일 (
              {rawFiles.length}
              개)
            </CardTitle>
          </CardHeader>
          <CardContent className="max-h-[200px] overflow-y-auto">
            {rawFiles.length > 0 ? (
              <ul className="space-y-1">
                {rawFiles.map(file => (
                  <li key={file.path} className="text-sm truncate">
                    {file.name}
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-muted-foreground text-center py-4">RAW 파일이 없습니다</p>
            )}
          </CardContent>
        </Card>
      </div>
      <Tabs className="w-full" defaultValue="organize">
        <TabsList className="grid grid-cols-2 mb-6">
          <TabsTrigger value="organize">파일 정리</TabsTrigger>
          <TabsTrigger value="group">그룹화 도구</TabsTrigger>
        </TabsList>
        <TabsContent className="space-y-4" value="organize">
          <Card>
            <CardHeader>
              <CardTitle>파일 타입별 정리</CardTitle>
              <CardDescription>일반 파일은 확장자별로, RAW와 XMP 파일은 raw 폴더로 분류합니다.</CardDescription>
            </CardHeader>
            <CardContent>
              <Button className="w-full" onClick={handleOrganizeByFileType}>파일 타입별 정리하기</Button>
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent className="space-y-4" value="group">
          <Card>
            <CardHeader>
              <CardTitle>사진 그룹화 도구</CardTitle>
              <CardDescription>다양한 기준으로 사진을 그룹화합니다.</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-2 mb-4">
                <Button onClick={() => handleGroupPhotos('date', '날짜')}>
                  날짜별로 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('lens', '렌즈')}>
                  렌즈별로 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('iso', 'ISO')}>
                  ISO별로 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('aperture', '조리개')}>
                  조리개별로 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('shutterSpeed', '셔터 스피드')}>
                  셔터 스피드별 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('focalLength', '초점 거리')}>
                  초점 거리별 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('cameraModel', '카메라 모델')}>
                  카메라 모델별 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('timeOfDay', '촬영 시간대')}>
                  시간대별 그룹화
                </Button>
                <Button onClick={() => handleGroupPhotos('gpsLocation', 'GPS 위치')}>
                  GPS 위치별 그룹화
                </Button>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
      {processingStatus && (
        <div className="mt-6 p-4 bg-[hsl(var(--muted))] rounded-md">
          <p className="text-center">{processingStatus}</p>
        </div>
      )}
    </div>
  );
};
