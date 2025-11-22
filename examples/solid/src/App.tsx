import { createSignal } from "solid-js";
import { Editor } from "@rockerrishabh/rich-text-editor-solid";

function App() {
  const [content, setContent] = createSignal("");

  return (
    <div>
      <h1>Solid JS Rich Text Editor</h1>
      <Editor
        initialContent="<p>Hello from Solid!</p>"
        onChange={(html) => setContent(html)}
      />
      <hr />
      <h2>Output:</h2>
      <div innerHTML={content()} />
    </div>
  );
}

export default App;
