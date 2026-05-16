<script setup lang="ts">
import { Copy, Check } from '@lucide/vue'
import { useShortener } from '@/composables/useShortener'
import Header from '@/components/Header.vue'

const { inputUrl, isLoading, errorMessage, result, copied, submit, copyUrl } = useShortener()

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') submit();
}
</script>

<template>
  <div class="min-h-dvh flex flex-col bg-background text-foreground">
    <Header />
    <main class="flex-1 flex flex-col items-center justify-center px-6 pt-24 pb-20 text-center">
      <h1 class="text-4xl md:text-5xl font-bold tracking-tight m-0 mb-3 max-w-xl text-foreground">
        url-shortener
      </h1>
      <p class="m-0 mb-10 max-w-sm text-sm text-foreground leading-relaxed">
        Fast and modern in-memory url-shortener
      </p>

      <div class="w-full max-w-xl flex flex-col gap-3 relative">
        <div class="flex gap-2 items-stretch">
          <input
              v-model="inputUrl"
              type="url"
              class="flex-1 px-4 py-3 text-sm bg-background border border-border rounded-lg focus:outline-hidden focus:ring-[3px] focus:ring-foreground/20"
              placeholder="https://example.com/some/very/long/path"
              autocomplete="off"
              spellcheck="false"
              :disabled="isLoading"
              @keydown="onKeydown"
          />
          <button
              class="px-5 py-3 bg-foreground text-background cursor-pointer text-sm font-medium rounded-lg disabled:pointer-events-none shrink-0"
              :disabled="isLoading || !inputUrl.trim()"
              @click="submit"
          >
            Shorten
          </button>
        </div>

        <div class="absolute top-full left-0 right-0 mt-3 flex flex-col gap-3">
          <p v-if="errorMessage" class="text-sm font-medium text-ink text-left px-1">
            {{ errorMessage }}
          </p>

          <div v-if="result" class="bg-background border border-border rounded-xl p-4 flex items-center justify-between gap-4 text-left">
            <div class="flex flex-col gap-0.5 min-w-0">
              <span class="text-xs font-bold tracking-wider text-foreground-faint">YOUR SHORT URL</span>
              <a
                  class="text-base font-semibold text-foreground inline-flex items-center gap-1.5 break-all"
              >
                {{ result.short_url }}
              </a>
              <span class="text-xs text-foreground-muted truncate mt-0.5">
                {{ result.original_url }}
              </span>
            </div>

            <button
                class="p-2.5 bg-background border border-border text-foreground hover:bg-background-raised rounded-lg shrink-0 cursor-pointer"
                @click="copyUrl(result.short_url)"
            >
              <Check v-if="copied" :size="16"/>
              <Copy v-else :size="16" />
            </button>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
