import { ref } from 'vue'
import { useClipboard } from '@vueuse/core'
import { shortenUrl, extractErrorMessage } from '@/api/shortener'
import type { ShortenResponse } from '@/types'

function isValidUrl(input: string): boolean {
    let parsed: URL
    try {
        parsed = new URL(input)
    } catch {
        return false
    }
    return parsed.protocol === 'http:' || parsed.protocol === 'https:'
}

export function useShortener() {
    const inputUrl = ref('')
    const isLoading = ref(false)
    const errorMessage = ref<string | null>(null)
    const result = ref<ShortenResponse | null>(null)

    const { copy, copied } = useClipboard({ legacy: true })

    async function submit(): Promise<void> {
        const url = inputUrl.value.trim()
        if (!url) return

        if (!isValidUrl(url)) {
            errorMessage.value = 'Please enter a valid URL'
            result.value = null
            return
        }

        isLoading.value = true
        errorMessage.value = null
        result.value = null

        try {
            result.value = await shortenUrl(url)
            inputUrl.value = ''
        } catch (err) {
            errorMessage.value = extractErrorMessage(err)
        } finally {
            isLoading.value = false
        }
    }

    function copyUrl(url: string): void {
        copy(url)
    }

    return {
        inputUrl,
        isLoading,
        errorMessage,
        result,
        copied,
        submit,
        copyUrl,
    }
}
