import { DocxDocument } from "../pkg/json2docx";

onmessage = function (e) {
  if (e.data.type === "convert") {
    let data = e.data.data;
    if (typeof data === "string") data = JSON.parse(data);
    doConvert(data);
  }
};

async function doConvert(data, config = {}) {
  const doc = DocxDocument.new();
  const bytes = await doc.from_js_chunks(data);
  const uint8 = Uint8Array.from(bytes);

  const blob = new Blob([uint8], {
    type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document;base64,",
  });

  postMessage({
    uid: config.uid || new Date().valueOf(),
    type: "ready",
    blob,
  });
}

postMessage({ type: "init" });
