<script setup lang="ts">
import { Link } from '@lucide/vue'
</script>

<template>
  <nav class="fixed top-0 left-0 right-0 z-10 border-b border-border">
    <div class="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between">
      <div class="flex items-center gap-0.5">
        <div class="flex items-center text-sm justify-center w-7 h-7 text-foreground">
          <Link :size="14" />
        </div>
        <span class="font-semibold text-sm tracking-tight">
          url-shortener
        </span>
      </div>
      <span class="text-sm">
        fast & simple
      </span>
    </div>
  </nav>
</template>
