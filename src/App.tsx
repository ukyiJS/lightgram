import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import './App.css';
import { useState } from 'react';

async function separateFilesByExtension(directoryPath: string): Promise<string> {
  try {
    return await invoke<string>('separate_files_by_extension', { dirPath: directoryPath });
  } catch (error) {
    console.error('Error separating file_features:', error);
    return error as string;
  }
}

function App() {
  const [message, setMessage] = useState<string>('');
  const handleSeparateFileClick = async () => {
    const path = await open({
      directory: true,
      multiple: false,
      title: '디렉토리 선택',
    });
    if (!path) return;

    const resultMessage = await separateFilesByExtension(path as string);
    setMessage(resultMessage);
  };

  const handleUndoClick = async () => {
    const resultMessage = await invoke<string>('undo_files');
    console.log('### resultMessage', resultMessage);
  };

  return (
    <main>
      <button onClick={handleSeparateFileClick}>raw, jpg 분리</button>
      <button onClick={handleUndoClick}>실행취소</button>
      {message}
    </main>
  );
}

export default App;
