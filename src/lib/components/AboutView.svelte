<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconShieldCheck, IconCode, IconDatabase } from "@tabler/icons-svelte";
</script>

<div class="max-w-3xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10">
  <div class="mb-10 text-center">
    <div class="inline-flex items-center justify-center w-20 h-20 bg-accent/10 text-accent rounded-full mb-6 border border-accent/20 shadow-[0_0_30px_rgba(var(--accent-rgb),0.2)]">
      <IconShieldCheck size={40} />
    </div>
    <h1 class="text-3xl font-bold text-text-primary mb-2">
      {$_("about_view.title")}
    </h1>
    <p class="text-text-muted text-lg">
      {$_("about_view.subtitle")}
    </p>
  </div>

  <div class="space-y-6">
    <!-- Card 1 -->
    <div class="p-6 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl flex gap-5 items-start transition-all hover:bg-surface/5">
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center shrink-0">
        <IconCode size={24} />
      </div>
      <div>
        <h3 class="text-lg font-bold text-text-primary mb-2">{$_("about_view.opensource_title")}</h3>
        <p class="text-text-muted leading-relaxed">
          {$_("about_view.opensource_desc")}
        </p>
      </div>
    </div>

    <!-- Card 2 -->
    <div class="p-6 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl flex gap-5 items-start transition-all hover:bg-surface/5">
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center shrink-0">
        <IconDatabase size={24} />
      </div>
      <div>
        <h3 class="text-lg font-bold text-text-primary mb-2">{$_("about_view.local_title")}</h3>
        <p class="text-text-muted leading-relaxed">
          {$_("about_view.local_desc")}
        </p>
      </div>
    </div>

    <!-- Card 3 -->
    <div class="p-6 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl flex gap-5 items-start transition-all hover:bg-surface/5">
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center shrink-0">
        <IconShieldCheck size={24} />
      </div>
      <div>
        <h3 class="text-lg font-bold text-text-primary mb-2">{$_("about_view.encryption_title")}</h3>
        <p class="text-text-muted leading-relaxed">
          {$_("about_view.encryption_desc")}
        </p>
      </div>
    </div>
  </div>
  <li class="py-2 my-4">
    <p class="text-sm">Para Actualizar este programa, debe volver a descargar la última versión desde <a class="cursor-pointer font-bold hover:underline text-accent" href="https://github.com/aaronlomas/boveda/releases/" target="_blank">Github Releases</a></p>
  </li>
</div>
