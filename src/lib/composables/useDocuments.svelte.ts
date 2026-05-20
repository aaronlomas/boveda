/**
 * @composable useDocuments
 * @description Composable reactivo para la gestión de la vista y documentos seguros.
 */
export function useDocuments() {
  let content = $state("");
  let currentView = $state<"selection" | "editor" | "import" | "list">("selection");
  let showSaveModal = $state(false);

  function startNew() {
    content = "";
    currentView = "editor";
  }

  function startExisting() {
    currentView = "import";
  }

  function startList() {
    currentView = "list";
  }

  function startSave() {
    showSaveModal = true;
  }

  function handleSave(title: string, description: string) {
    console.log("Saving note:", { title, description, content });
    showSaveModal = false;
    // Aquí irá la lógica de cifrado real
  }

  function goBack() {
    currentView = "selection";
  }

  return {
    get content() { return content; },
    set content(val) { content = val; },
    get currentView() { return currentView; },
    get showSaveModal() { return showSaveModal; },
    set showSaveModal(val) { showSaveModal = val; },
    startNew,
    startExisting,
    startList,
    startSave,
    handleSave,
    goBack,
  };
}
