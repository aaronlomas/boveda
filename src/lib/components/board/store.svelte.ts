import { getAlignment, getCommandState, getFontSizeAtCaret } from "./utils/selection";
import { execBoardCommand, setFontSize } from "./utils/formatting";

export class BoardStore {
  // State
  isBold = $state(false);
  isItalic = $state(false);
  isUnderline = $state(false);
  isStrikethrough = $state(false);
  isListUl = $state(false);
  isListOl = $state(false);
  textAlign = $state<"left" | "center" | "right">("left");
  currentSize = $state(14);
  pasteWarning = $state("");
  content = $state("");

  // Refs (to be bound by the component)
  editorRef = $state<HTMLDivElement | undefined>();

  constructor(initialContent: string = "") {
    this.content = initialContent;
  }

  updateState() {
    this.isBold = getCommandState("bold");
    this.isItalic = getCommandState("italic");
    this.isUnderline = getCommandState("underline");
    this.isStrikethrough = getCommandState("strikeThrough");
    this.isListUl = getCommandState("insertUnorderedList");
    this.isListOl = getCommandState("insertOrderedList");
    this.textAlign = getAlignment();
    const detected = getFontSizeAtCaret();
    if (detected !== null) this.currentSize = detected;
  }

  handleCommand(cmd: string, val?: string) {
    execBoardCommand(cmd, val);
    this.updateState();
    this.syncContent();
  }

  handleFontSize(size: number) {
    setFontSize(size);
    this.currentSize = size;
    this.syncContent();
  }

  syncContent() {
    if (this.editorRef) {
      this.content = this.editorRef.innerHTML;
    }
  }

  setPasteWarning(msg: string) {
    this.pasteWarning = msg;
  }

  clearPasteWarning() {
    this.pasteWarning = "";
  }
}
