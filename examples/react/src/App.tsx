import { useState } from "react";
import "./App.css";
import { Editor } from "@rockerrishabh/rich-text-editor-react";

function App() {
  const [content, setContent] = useState("");

  return (
    <div className="app-root">
      <h1>Rich Text Editor — React Example</h1>
      <Editor
        className="rte-editor"
        initialContent="<p>Hello from the local example!</p>"
        placeholder="Start typing..."
        onChange={(c: string) => setContent(c)}
      />

      <section className="output">
        <h2>Editor Output</h2>
        <pre>{content}</pre>
      </section>
    </div>
  );
}

export default App;
