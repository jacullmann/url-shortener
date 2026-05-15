<script setup lang="ts">
import { ArrowRight, Copy, Check, ExternalLink } from '@lucide/vue'
import { useShortener } from '@/composables/useShortener'
import Header from '@/components/Header.vue'

const { inputUrl, isLoading, errorMessage, result, copied, submit, copyUrl } = useShortener()

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') submit()
}
</script>

<template>
  <div class="min-h-dvh flex flex-col bg-[--color-surface]">
    <Header />
    <main class="flex-1 flex flex-col items-center justify-center px-6 pt-24 pb-20 text-center">

      <h1 class="text-5xl font-semibold tracking-tight text-[--color-ink] m-0 mb-4 leading-[1.1] max-w-xl">
        url-shortener
      </h1>
      <p class="text-[length:--font-size-base] text-[--color-ink-muted] m-0 mb-12 max-w-sm leading-relaxed">
        Fast and modern in-memory url-shortener
      </p>

      <div class="w-full max-w-xl flex flex-col gap-3">
        <div class="flex gap-2.5 items-stretch">
          <input
              v-model="inputUrl"
              type="url"
              class="input input--lg"
              :class="{ 'input--error': !!errorMessage }"
              placeholder="https://example.com/some/very/long/path"
              autocomplete="off"
              spellcheck="false"
              :disabled="isLoading"
              @keydown="onKeydown"
          />
          <button
              class="btn btn--primary btn--lg"
              :disabled="isLoading || !inputUrl.trim()"
              @click="submit"
          >
            <span v-if="isLoading" class="spinner" />
            <template v-else>
              Shorten
              <ArrowRight :size="16" />
            </template>
          </button>
        </div>

        <p v-if="errorMessage" class="error-text text-left px-1">
          {{ errorMessage }}
        </p>

        <Transition
            enter-active-class="transition-all duration-200 ease-out"
            enter-from-class="opacity-0 translate-y-1"
            enter-to-class="opacity-100 translate-y-0"
        >
          <div v-if="result" class="card p-4 flex items-center justify-between gap-4 text-left">
            <div class="flex flex-col gap-1 min-w-0">
              <span class="label">your short url</span>
              <a
                  :href="result.short_url"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-[length:--font-size-base] font-semibold text-[--color-ink] no-underline inline-flex items-center gap-1.5 hover:opacity-60 transition-opacity"
              >
                {{ result.short_url }}
                <ExternalLink :size="13" class="opacity-40 shrink-0" />
              </a>
              <span class="mono text-[--color-ink-faint] truncate">{{ result.original_url }}</span>
            </div>
            <button
                class="btn btn--ghost btn--icon-lg shrink-0"
                :title="copied ? 'Copied!' : 'Copy'"
                @click="copyUrl(result.short_url)"
            >
              <Check v-if="copied" :size="16" />
              <Copy v-else :size="16" />
            </button>
          </div>
        </Transition>
      </div>
    </main>
  </div>
</template>
