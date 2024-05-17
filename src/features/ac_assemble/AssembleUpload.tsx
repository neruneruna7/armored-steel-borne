import React, { useCallback } from 'react';
import { dummyAcAsmPostReq } from './dummyData';



export default function Upload() {
  const handleUpload = useCallback(async () => {
    const response = await fetch('/asm/create', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(dummyAcAsmPostReq), // ここに先ほど作成したダミーデータを使用
    });

    if (!response.ok) {
      // エラーハンドリング
      console.error('Upload failed');
    }
  }, []);

  return (
    <div className="flex flex-col gap-8 pt-10 justify-center bg-slate-50 items-center pb-10 overflow-y-scroll">
      <button onClick={handleUpload} className='border-4  w-80 h-60'>Upload</button>
    </div>
  );
}