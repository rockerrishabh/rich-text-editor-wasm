import { RichTextEditor } from "@rockerrishabh/rich-text-editor-core";
import "@rockerrishabh/rich-text-editor-core/dist/style.css";

const editorContainer = document.getElementById("editor-container");

if (editorContainer) {
  const editor = new RichTextEditor(editorContainer);
  editor.focus();
}
