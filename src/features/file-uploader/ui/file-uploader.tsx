import type { ChangeEvent } from 'react';
import { useRef, useState } from 'react';

import { Button } from '@/shared/ui/button';
import { Input } from '@/shared/ui/input';

interface FileUploaderProps {
  acceptTypes?: string;
  label?: string;
  onFileSelect?(file: File): void;
}

export const FileUploader = ({
  onFileSelect,
  acceptTypes = '.jpg,.jpeg,.png,.raw,.arw,.cr2,.nef',
  label = '파일 선택',
}: FileUploaderProps) => {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const [file] = e.target.files;

      setSelectedFile(file);
      onFileSelect?.(file);
    }
  };

  const handleButtonClick = () => {
    fileInputRef.current?.click();
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        <Input
          ref={fileInputRef}
          accept={acceptTypes}
          className="hidden"
          type="file"
          onChange={handleFileChange}
        />
        <Button
          className="w-full"
          type="button"
          variant="outline"
          onClick={handleButtonClick}
        >
          {label}
        </Button>
      </div>
      {selectedFile && (
        <p className="text-sm text-[hsl(var(--muted-foreground))]">
          선택된 파일:
          {' '}
          {selectedFile.name}
        </p>
      )}
    </div>
  );
};
