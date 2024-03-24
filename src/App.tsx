import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import './App.css';
import { useState } from 'react';

function App() {
  const [separateMessage, setSeparateMessage] = useState<string>('');
  const handleSeparateFileClick = async () => {
    const dirPath = await open({
      directory: true,
      multiple: false,
      title: '디렉토리 선택',
    });
    if (!dirPath) return;

    const duration = await invoke<string>('separate_files_by_extension', { dirPath });
    setSeparateMessage(`분리가 완료되었습니다. (${duration}초)`);
  };

  const handleUndoClick = async () => {
    await invoke<string>('undo_files');
  };

  return (
    <main>
      <button onClick={handleSeparateFileClick}>raw, jpg 분리</button>
      <button onClick={handleUndoClick}>실행취소</button>
      {separateMessage}
    </main>
  );
}

export default App;
