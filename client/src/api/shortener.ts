import axios, { AxiosError } from 'axios'
import type { ShortenResponse, ErrorResponse } from '@/types'

const http = axios.create({
    baseURL: '/',
    headers: { 'Content-Type': 'application/json' },
})

export async function shortenUrl(url: string): Promise<ShortenResponse> {
    const response = await http.post<ShortenResponse>('/shorten', { url })
    return response.data
}

export function extractErrorMessage(error: unknown): string {
    if (error instanceof AxiosError && error.response?.data) {
        const data = error.response.data as ErrorResponse
        if (data.error) return data.error
    }
    return 'Something went wrong.'
}
