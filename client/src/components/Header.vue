<script setup lang="ts">
import { Link2 } from '@lucide/vue'
</script>

<template>
  <nav class="fixed top-0 left-0 right-0 z-10 border-b border-[--color-border] bg-[--color-surface]/90 backdrop-blur-sm">
    <div class="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="flex items-center justify-center w-7 h-7 bg-[--color-ink] text-[--color-surface] rounded-[--radius-sm]">
          <Link2 :size="14" />
        </div>
        <span class="font-semibold text-[length:--font-size-base] tracking-tight text-[--color-ink]">
          url-shortener
        </span>
      </div>
      <span class="text-[length:--font-size-xs] text-[--color-ink-faint] font-mono">
        fast & performant
      </span>
    </div>
  </nav>
</template>
