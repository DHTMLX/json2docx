import { DocxDocument } from "../pkg/json2docx.js";

export function convert(data) {
  if (typeof data == "string") {
    data = JSON.parse(data);
  }

  const doc = DocxDocument.new();
  const result = doc.from_js_chunks(data);

  const blob = new Blob([result], {
    type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document;base64,",
  });

  return blob;
}
