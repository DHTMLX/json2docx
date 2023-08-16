import { DocxDocument } from '../pkg/json2docx';


onmessage = function(e) {
    if (e.data.type === "convert") {
        let data = e.data.data;
        if (typeof data === "string")
            data = JSON.parse(data);
        doConvert(data);
    }
}

function doConvert(data, config = {}){
    const doc = DocxDocument.new();
    const result = doc.from_js_chunks(data);
    
    const blob = new Blob([result], {
        type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet;base64,"
    });

    postMessage({
        uid: config.uid || (new Date()).valueOf(),
        type: "ready",
        blob
    });
}

postMessage({ type:"init" });
