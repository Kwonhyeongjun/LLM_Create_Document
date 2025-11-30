import { Editor } from "@toast-ui/react-editor";
import "@toast-ui/editor/dist/toastui-editor.css";
import { exportToDocx } from "../utils/converter";
import "../styles/EditorPage.css";

const EditorPage = ({ editorRef, editorContent, uploadedFile }) => {
  const getFileName = (name) => name?.replace(/\.txt$/, "");

  const handleSendClick = () => {
    if (!editorRef || !editorRef.current) return;

    const editorInstance = editorRef.current.getInstance();
    const html = editorInstance.getHTML(); // 서식 포함 HTML

    if (onSend) {
      onSend({
        fileName: uploadedFile ? uploadedFile.name : "document.txt",
        html,
      });
    }
  };

  return (
    <div className="editor-container">
      <h2 className="editor-title">
        {uploadedFile?.name} 파일을 문서(.docx)로 변환 전 검토•수정하세요.
      </h2>

      <Editor
        ref={editorRef}
        height="500px"
        initialEditType="wysiwyg"
        initialValue={editorContent}
        previewStyle="vertical"
        hideModeSwitch={true}
        toolbarItems={[
          ["bold", "italic", "strike"],
          ["ul", "ol"],
        ]}
      />

      <div style={{ marginTop: "16px", textAlign: "right" }}>
        <button
          onClick={() => exportToDocx(
            editorRef,
            `${getFileName(uploadedFile?.name)}.docx`
          )}
          style={{
            padding: "10px 20px",
            borderRadius: "8px",
            border: "none",
            cursor: "pointer",
            fontWeight: "bold",
          }}
        >
          DOCX로 보내기 (Send)
        </button>
      </div>
    </div>
  );
};

export default EditorPage;
