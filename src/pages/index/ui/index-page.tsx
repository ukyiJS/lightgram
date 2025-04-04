import { invoke } from '@tauri-apps/api/core';

import reactLogo from '/vite.svg';

import { useState } from 'react';

function IndexPage() {
  const [name, setName] = useState('');
  const [greetMsg, setGreetMsg] = useState('');

  async function greet() {
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>
      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img alt="Vite logo" className="logo vite" src="/vite.svg" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img alt="Tauri logo" className="logo tauri" src="/tauri.svg" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img alt="React logo" className="logo react" src={reactLogo} />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <form
        className="row"
        onSubmit={e => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          placeholder="Enter a name..."
          onChange={e => setName(e.target.value)}
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default IndexPage;
