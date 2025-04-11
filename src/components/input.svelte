<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { m } from '$lib/paraglide/messages.js'; // Paraglide import
    export let message: string;

    const dispatch = createEventDispatcher();

    function handleInput(e: Event) {
        const target = e.currentTarget as HTMLTextAreaElement;
        dispatch('messageChange', target.value);
        target.style.height = 'auto';
        target.style.overflowY = target.scrollHeight > 200 ? 'auto' : 'hidden';
        target.style.height = Math.min(target.scrollHeight, 200) + 'px';
    }
</script>

<style>
    textarea::-webkit-scrollbar {
        width: 4px;
    }
    textarea::-webkit-scrollbar-track {
        background: transparent;
    }
    textarea::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.25);
        border-radius: 3px;
    }
</style>

<div class="relative w-full max-w-2xl rounded-xl shadow-lg bg-[#2a2a2a]/80 border border-transparent p-4 pb-16">
    <!-- Textarea -->
    <textarea
            bind:value={message}
            rows="1"
            placeholder={m.ask_anything()}
            class="w-full h-auto max-h-[200px] text-sm resize-none bg-transparent text-white border border-transparent focus:outline-none focus:ring-0 focus:border-transparent overflow-y-hidden"
            on:input={handleInput}
    ></textarea>

    <!-- Button Row -->
    <div class="absolute bottom-4 left-4 flex items-center gap-2">
        <!-- + Button -->
        <button
                aria-label={m.add()}
                class="w-9 h-9 bg-[#3a3a3a] hover:bg-[#505050] text-white rounded-full flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
        </button>

        <!-- Search Button -->
        <button class="flex items-center gap-1 bg-[#3a3a3a] hover:bg-[#505050] text-white px-3 py-1.5 rounded-full text-sm font-medium">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                <path d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5a17.92 17.92 0 0 1-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"/>
            </svg>
            <span>{m.search()}</span>
        </button>

        <!-- Reason Button -->
        <button class="flex items-center gap-1 bg-[#3a3a3a] hover:bg-[#505050] text-white px-3 py-1.5 rounded-full text-sm font-medium">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                <path d="M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18"/>
            </svg>
            <span>{m.reason()}</span>
        </button>

        <!-- 3-Dot Horizontal Button -->
        <button
                aria-label={m.more_options()}
                class="w-9 h-9 bg-[#3a3a3a] hover:bg-[#505050] text-white rounded-full flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path d="M6 10a2 2 0 1 1-4 0a2 2 0 0 1 4 0Zm6 0a2 2 0 1 1-4 0a2 2 0 0 1 4 0Zm4 2a2 2 0 1 0 0-4a2 2 0 0 0 0 4Z"/>
            </svg>
        </button>
    </div>

    <!-- Submit Button (Arrow Up) -->
    <button
            aria-label={m.submit()}
            class="absolute bottom-4 right-4 text-black bg-white hover:bg-gray-200 p-2 rounded-full">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
        </svg>
    </button>
</div>
