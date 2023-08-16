import { DocxDocument } from '../pkg/json2docx.js';

export function convert(data) {

  if (typeof data == "string") {
    data = JSON.parse(data);
  }

  const doc = DocxDocument.new();
  const result = doc.from_chunks();

  console.log(result);

  return;

  // const result = import_to_xlsx(data);
  // const blob = new Blob([result], {
  //     type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet;base64,"
  // });

  // return blob;
}
