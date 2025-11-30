import { saveAs } from "file-saver";
import { Document, Packer, Paragraph, TextRun, AlignmentType, UnderlineType } from "docx";

const DEFAULT_FONT = "맑은 고딕";
const DEFAULT_SIZE = 20; // 10pt
const LIST_LEVEL = 0;

export const exportToDocx = async (editorRef, fileName = "document.docx") => {
  if (!editorRef.current) return;

  const editor = editorRef.current.getInstance();
  const htmlDoc = new DOMParser().parseFromString(editor.getHTML(), "text/html");

  const children = Array.from(htmlDoc.body.childNodes)
    .map(convertNode)
    .flat()
    .filter(Boolean);

  const doc = new Document({
    numbering: {
      config: [
        {
          reference: "number-list",
          levels: [{ level: LIST_LEVEL, format: "decimal", text: "%1.", alignment: AlignmentType.START }],
        },
        {
          reference: "bullet-list",
          levels: [{ level: LIST_LEVEL, format: "bullet", text: "•", alignment: AlignmentType.START }],
        },
      ],
    },
    sections: [{ children }],
  });

  saveAs(await Packer.toBlob(doc), fileName);
};

// ========= HTML 변환 =========
function convertNode(node) {
  if (!node || node.nodeType === 3) return null;

  const tag = node.tagName?.toLowerCase();

  if (tag === "ul") return convertList(node, "bullet");
  if (tag === "ol") return convertList(node, "number");

  // 모든 단락
  return new Paragraph({ children: extractRunsDeep(node) });
}

// ========= 리스트 처리 =========
function convertList(node, type) {
  return Array.from(node.childNodes)
    .filter((li) => li.tagName?.toLowerCase() === "li")
    .map(
      (li) =>
        new Paragraph({
          children: extractRunsDeep(li),
          numbering: { reference: type === "bullet" ? "bullet-list" : "number-list", level: LIST_LEVEL },
        })
    );
}

// ========= 중첩 스타일 지원 TextRun 추출 =========
function extractRunsDeep(node, inheritedStyle = {}) {
  if (node.nodeType === 3) {
    return [
      new TextRun({
        text: node.textContent,
        font: DEFAULT_FONT,
        size: DEFAULT_SIZE,
        ...inheritedStyle,
      }),
    ];
  }

  if (node.nodeType !== 1) return [];

  const tag = node.tagName.toLowerCase();
  const style = { ...inheritedStyle };

  switch (tag) {
    case "strong":
      style.bold = true;
      break;
    case "em":
      style.italics = true;
      break;
    case "u":
      style.underline = { type: UnderlineType.SINGLE };
      break;
    case "del":
      style.strike = true;
      break;
  }

  return Array.from(node.childNodes).flatMap((child) => extractRunsDeep(child, style));
}
