import "../styles/UploadPage.css";

const UploadPage = ({
  handleBrowseClick,
  onFileSelect,
  handleDragEnter,
  handleDragOver,
  handleDragLeave,
  handleDrop,
  dropzoneClass,
  fileInputRef,
}) => {
  return (
    <div className="container">
      <h2>텍스트 파일을 업로드 해주세요.</h2>

      <section
        className={dropzoneClass}
        onClick={handleBrowseClick}
        onDragEnter={handleDragEnter}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        <div className="dropzone-instructions">
          <span className="dropzone-icon" style={{ fontSize: "2em" }}>
            <img
              src="/src/assets/cloud.png"
              style={{ width: "1em", height: "1em" }}
              alt="cloud"
            />
          </span>
          <div>
            <span>Drag and drop file here</span>
            <span>Limit 50MB • TXT only</span>
          </div>
        </div>

        <button
          className="secondary-button browse-button"
          onClick={(e) => {
            e.stopPropagation();
            handleBrowseClick();
          }}
        >
          Browse files
        </button>

        <input
          type="file"
          accept=".txt"
          ref={fileInputRef}
          onChange={onFileSelect}
          style={{ display: "none" }}
          multiple={false}
        />
      </section>
    </div>
  );
};

export default UploadPage;
