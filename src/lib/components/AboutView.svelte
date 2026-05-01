<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconShieldCheck, IconCode, IconDatabase } from "@tabler/icons-svelte";
</script>

<div class="max-w-[800px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10">
  <div class="mb-10 text-center">
    <div class="inline-flex items-center justify-center w-20 h-20 bg-accent/10 text-accent rounded-full mb-6 border border-accent/20 shadow-[0_0_30px_rgba(var(--accent-rgb),0.2)]">
      <IconShieldCheck size={40} />
    </div>
    <h1 class="text-[32px] font-bold text-white mb-2">
      {$_("about_view.title")}
    </h1>
    <p class="text-text-muted text-[16px]">
      {$_("about_view.subtitle")}
    </p>
  </div>

  <div class="space-y-6">
    <!-- Card 1 -->
    <div class="p-6 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl flex gap-5 items-start transition-all hover:bg-white/5">
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
    <div class="p-6 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl flex gap-5 items-start transition-all hover:bg-white/5">
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
    <div class="p-6 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl flex gap-5 items-start transition-all hover:bg-white/5">
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
  <li>
    <p>Para Actualizar este programa, debe volver a descargar la última versión desde <a href="https://github.com/aaronlomas/boveda/releases/">GitHub Releases</a></p>
  </li>
</div>
