<script lang="ts">
    import Greet from "$lib/Greet.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let isRecording = $state(false);
    let mediaRecorder: MediaRecorder | null = $state(null);
    let chunks: BlobPart[] = $state([]);
    let audioDataUrl: string | null = $state(null);
    let isSaving = $state(false);

    async function toggleRecording() {
        if (!isRecording) {
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
            chunks = [];
            mediaRecorder = new MediaRecorder(stream, { mimeType: "audio/webm" });
            mediaRecorder.ondataavailable = (e) => {
                if (e.data && e.data.size > 0) {
                    chunks.push(e.data);
                }
            };
            mediaRecorder.onstop = async () => {
                const blob = new Blob(chunks, { type: "audio/webm" });
                const arrayBuffer = await blob.arrayBuffer();
                const bytes = new Uint8Array(arrayBuffer);
                isSaving = true;
                try {
                    const dataUrl = await invoke<string>("save_audio_file", {
                        bytes: Array.from(bytes),
                        filename: null,
                    });
                    audioDataUrl = dataUrl;
                } catch (e) {
                    console.error(e);
                } finally {
                    isSaving = false;
                }
            };
            mediaRecorder.start();
            isRecording = true;
        } else {
            mediaRecorder?.stop();
            // stop all tracks
            mediaRecorder?.stream.getTracks().forEach((t) => t.stop());
            isRecording = false;
        }
    }

    function playSaved() {
        const el = document.getElementById("saved-audio") as HTMLAudioElement | null;
        el?.play();
    }
</script>

<h1>Welcome to SvelteKit ❤️</h1>
<Greet />

<div style="margin-top: 16px; display: flex; align-items: center; gap: 12px;">
    <button onclick={toggleRecording} disabled={isSaving}>
        {#if isRecording}
            Stop recording
        {:else}
            Start recording
        {/if}
    </button>
    {#if isRecording}
        <span>Enregistrement en cours…</span>
    {/if}
    {#if audioDataUrl}
        <button title="Play" onclick={playSaved}>▶️</button>
        <audio id="saved-audio" src={audioDataUrl} controls style="display:none"></audio>
        <small>Audio ready for playback</small>
    {/if}
</div>
