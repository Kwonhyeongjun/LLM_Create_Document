import { useState, useRef } from "react";
import UploadPage from "./components/UploadPage.jsx";
import EditorPage from "./components/EditorPage.jsx";
import "./App.css";

function App() {
  const [uploadedFile, setUploadedFile] = useState(null);
  const [showEditor, setShowEditor] = useState(false);
  const [editorContent, setEditorContent] = useState("");
  const [isDragging, setIsDragging] = useState(false);

  const fileInputRef = useRef(null);
  const editorRef = useRef(null);

  const handleFileChange = async (files) => {
    if (!files || files.length === 0) return;

    const file = files[0];

    if (!file.name.endsWith(".txt")) {
      alert("텍스트 파일만 업로드할 수 있습니다.");
      return;
    }

    const maxSize = 50 * 1024 * 1024;
    if (file.size > maxSize) {
      alert("파일 용량이 50MB를 초과했습니다.");
      return;
    }

    setUploadedFile(file);

    const reader = new FileReader();
    reader.onload = async (e) => {
      const textContent = e.target.result;
      setEditorContent(textContent);
      setShowEditor(true);
    };

    reader.readAsText(file, "utf-8");
    setIsDragging(false);
  };

  const onFileSelect = (event) => {
    handleFileChange(event.target.files);
    event.target.value = null;
  };

  // 드래그 앤 드롭
  const handleDragEnter = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };
  const handleDragOver = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };
  const handleDragLeave = (e) => {
    e.preventDefault();
    setIsDragging(false);
  };
  const handleDrop = (e) => {
    e.preventDefault();
    setIsDragging(false);
    handleFileChange(e.dataTransfer.files);
  };

  const handleBrowseClick = () => fileInputRef.current.click();
  const dropzoneClass = `dropzone ${isDragging ? "dragover" : ""}`;

  return showEditor ? (
    <EditorPage
      editorRef={editorRef}
      editorContent={editorContent}
      uploadedFile={uploadedFile}
    />
  ) : (
    <UploadPage
      handleBrowseClick={handleBrowseClick}
      onFileSelect={onFileSelect}
      handleDragEnter={handleDragEnter}
      handleDragOver={handleDragOver}
      handleDragLeave={handleDragLeave}
      handleDrop={handleDrop}
      dropzoneClass={dropzoneClass}
      fileInputRef={fileInputRef}
    />
  );
}

export default App;
