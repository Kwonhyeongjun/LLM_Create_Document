import { Editor } from "@toast-ui/react-editor";
import "@toast-ui/editor/dist/toastui-editor.css";

const EditorPage = ({ editorRef, editorContent, uploadedFile }) => {
  const sanitizeContent = () => {
    const editorInstance = editorRef.current.getInstance();
    let html = editorInstance.getHTML();

    html = html.replace(/<code>[\s\S]*?<\/code>/gi, "");
    html = html.replace(/<blockquote>[\s\S]*?<\/blockquote>/gi, "");

    editorInstance.setHTML(html);
  };

  return (
    <div className="container">
      <h2 style={{ textAlign: "center" }}>
        {uploadedFile?.name} 파일을 문서(.docx)로 변환 전 검토•수정하세요.
      </h2>

      <div style={{ marginTop: "20px" }}>
        <Editor
          ref={editorRef}
          height="500px"
          initialEditType="wysiwyg"
          initialValue={editorContent}
          previewStyle="vertical"
          hideModeSwitch={true}
          toolbarItems={[
            ["heading", "bold", "italic", "strike"],
            ["ul", "ol"],
            ["link"],
          ]}
          events={{
            blur: sanitizeContent,
            change: sanitizeContent,
          }}
        />
      </div>
    </div>
  );
};

export default EditorPage;
