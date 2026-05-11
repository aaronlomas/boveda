<script lang="ts">
  import { globalState } from "$lib/stores/stores.svelte";
  import { lockVault } from "$lib/utils/tauri";
  import { modal } from "$lib/stores/modal.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { _ } from "svelte-i18n";
  import { toast } from "$lib/stores/toast.svelte";
  import {
    IconShieldHalfFilled,
    IconLayoutGrid,
    IconFiles,
    IconDatabaseImport,
    IconArchive,
    IconInfoCircle,
    IconSettings,
    IconLogout,
    IconChevronLeft,
  } from "@tabler/icons-svelte";

  let navItems = $derived([
    {
      icon: IconLayoutGrid,
      label: $_("sidebar.general"),
      id: "general",
      action: () => (globalState.activeView = "general"),
    },
    {
      icon: IconFiles,
      label: $_("sidebar.documents"),
      id: "documents",
      action: () => (globalState.activeView = "documents"),
    },
    {
      icon: IconArchive,
      label: $_("dashboard.export_db"),
      id: "export",
      action: () => handleExportSecure(),
    },
    {
      icon: IconDatabaseImport,
      label: $_("sidebar.import_db"),
      id: "import",
      action: () => handleImport(),
    },
    {
      icon: IconInfoCircle,
      label: $_("sidebar.about"),
      id: "about",
      action: () => (globalState.activeView = "about"),
    },
    {
      icon: IconSettings,
      label: $_("sidebar.settings"),
      id: "settings",
      action: () => (globalState.activeView = "settings"),
    },
  ]);

  async function handleImport() {
    try {
      const filePath = await open({
        title: $_("global.select_db_title"),
        filters: [
          { name: "Bóveda Vaults", extensions: ["bvda", "db", "pack", "bvda.pack"] },
        ],
      });
      
      if (!filePath) return;

      if (filePath.endsWith(".pack") || filePath.endsWith(".bvda.pack")) {
        // Secure Package Import
        modal.openImportPackage({
          title: "import_pack.title",
          desc: "import_pack.desc",
          buttonText: "import_pack.button",
          onconfirm: async (password, strategy) => {
            try {
              await invoke("import_secure_package", { srcPath: filePath, password, strategy });
              toast.success(strategy === 'replace' ? $_("import_pack.success_replace") : $_("import_pack.success_merge"));
            } catch (e: any) {
              console.error("Secure import failed:", e);
              toast.error($_("global.error_import") + ": " + e.toString());
            }
          }
        });
      } else {
        // Legacy DB Import (SQLite file replacement)
        modal.openConfirm({
          title: $_("sidebar.import_confirm_title"),
          message: $_("sidebar.import_confirm_message"),
          confirmText: $_("sidebar.import_confirm_button"),
          type: "danger",
          onconfirm: async () => {
            try {
              await invoke("import_db", { srcPath: filePath });
              toast.success($_("sidebar.import_confirm_button"));
            } catch (e) {
              console.error("Import failed:", e);
              toast.error($_("global.error_import"));
            }
          },
        });
      }
    } catch (e) {
      console.error("Import selection failed:", e);
    }
  }

  async function handleExportSecure() {
    modal.openExportPackage({
      title: $_("dashboard.export_db_title"),
      desc: "Crea un paquete blindado (.bvda.pack)",
      buttonText: "Generar Paquete",
      onconfirm: async (password) => {
        try {
          const filePath = await save({
            title: $_("dashboard.export_db_title"),
            defaultPath: "Boveda_Export.bvda.pack",
            filters: [
              { name: "Bóveda Secure Package", extensions: ["pack", "bvda.pack"] },
            ],
          });

          if (filePath) {
            await invoke("export_secure_package", {
              destPath: filePath,
              password: password,
            });
            toast.success(
              $_("dashboard.export_success", { values: { path: filePath } }),
            );
          }
        } catch (e) {
          console.error("Export failed:", e);
          toast.error($_("dashboard.export_error"));
        }
      },
    });
  }

  function toggle() {
    globalState.sidebarCollapsed = !globalState.sidebarCollapsed;
  }

  async function logout() {
    try {
      await lockVault();
      globalState.isUnlocked = false;
    } catch (e) {
      console.error("Logout error:", e);
    }
  }
</script>

<aside
  class="h-screen bg-panel/30 border-r border-surface/8 transition-full overflow-hidden backdrop-blur-2xl flex flex-col py-4 px-2"
  style="width: {globalState.sidebarCollapsed
    ? 'var(--sidebar-collapsed)'
    : 'var(--sidebar-w)'}; min-width: {globalState.sidebarCollapsed
    ? 'var(--sidebar-collapsed)'
    : 'var(--sidebar-w)'};"
>
  <!-- Brand -->
  <div
    class="flex items-center gap-3 px-1 pb-5 border-b border-surface/8 mb-3 overflow-hidden whitespace-nowrap"
  >
    <div
      class="text-2xl shrink-0 w-10 h-10 flex items-center justify-center bg-transparent rounded-sm border-none"
    >
      <IconShieldHalfFilled size={40} />
    </div>
    {#if !globalState.sidebarCollapsed}
      <div class="flex flex-col">
        <span
          class="text-base font-bold text-text-primary block pointer-events-none"
          >Bóveda</span
        >
        <span
          class="text-xs text-text-muted uppercase tracking-wider pointer-events-none"
          >{$_("sidebar.my_credentials")}</span
        >
      </div>
    {/if}
  </div>

  <!-- Nav items -->
  <nav class="flex-1 flex flex-col gap-1">
    {#each navItems as item}
      {@const Icon = item.icon}
      <button
        class="nav-item-btn {globalState.activeView === item.id
          ? 'active'
          : ''}"
        onclick={item.action ?? undefined}
        data-tooltip={globalState.sidebarCollapsed ? item.label : undefined}
      >
        <div class="shrink-0 w-5 flex justify-center">
          <Icon size={20} />
        </div>
        {#if !globalState.sidebarCollapsed}
          <span class="flex-1">{item.label}</span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Logout button -->
  <button
    class="mt-auto flex justify-center items-center gap-3 p-2.5 px-3 border-none rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-xs transition-full whitespace-nowrap overflow-hidden w-full text-left hover:bg-danger/10 hover:text-text-primary"
    onclick={logout}
    data-tooltip={globalState.sidebarCollapsed
      ? $_("sidebar.logout")
      : undefined}
  >
    <div class="shrink-0 w-5 flex justify-center">
      <IconLogout size={20} />
    </div>
    {#if !globalState.sidebarCollapsed}
      <span class="flex-1">{$_("sidebar.logout")}</span>
    {/if}
  </button>

  <!-- Collapse toggle -->
  <button
    class="flex items-center gap-2.5 mb-6 p-4 border-none border-t border-surface/8 rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-xs transition-all whitespace-nowrap overflow-hidden w-full mt-2 pt-4 hover:text-text-secondary"
    onclick={toggle}
    aria-label="Toggle sidebar"
  >
    <div
      class="transition-transform duration-300 shrink-0 w-5 flex justify-center"
      class:rotate-180={globalState.sidebarCollapsed}
    >
      <IconChevronLeft size={20} />
    </div>
    {#if !globalState.sidebarCollapsed}
      <span class="text-xs">{$_("sidebar.collapse")}</span>
    {/if}
  </button>
</aside>

