import { DocxDocument } from "../pkg/json2docx.js";

export async function convert(data) {
  if (typeof data == "string") {
    data = JSON.parse(data);
  }

  const doc = DocxDocument.new();
  const bytes = await doc.from_js_chunks(data);
  const uint8 = Uint8Array.from(bytes);

  const blob = new Blob([uint8], {
    type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document;base64,",
  });

  return blob;
}
