``` mermaid
flowchart TD

file_metadata/:file_key

session/state

user/state?file_key=xx

roles/file/:file_key

files/:file_key/realtime_token
```

https://www.figma2json.com/


``` typescript

interface GetFileMetadataRes {
  name: string;
  canvas_url: string;
  file_key: string;
  feature_flags: {
    [feature: string]: boolean
  },
  source_file_key: null
}

function getFileMetadata(key) {

}

interface Figma {
  root: {
    type: "NODE_CHANGES",
    sessionID: 0,
    ackID: 0,
    nodeChanges: NodeChange[],
    blobs: string[],
  }
}

interface NodeChange {
  guid:{
    "sessionID": number,
    "localID": number,
  },
  type: "DOCUMENT",
  phase: "CREATED"
}

```
