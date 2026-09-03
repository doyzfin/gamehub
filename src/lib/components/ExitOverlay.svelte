<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { settings } from '$lib/stores/settings';

  let exitProgress = $state(0);
  let isHoldingEsc = $state(false);
  let animationFrame: number;
  let startTime: number;
  
  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape' && !isHoldingEsc) {
      isHoldingEsc = true;
      startTime = performance.now();
      updateProgress();
    }
  }
  
  function handleKeyUp(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      isHoldingEsc = false;
      exitProgress = 0;
      cancelAnimationFrame(animationFrame);
    }
  }
  
  function updateProgress() {
    if (!isHoldingEsc) return;
    
    const now = performance.now();
    const elapsed = now - startTime;
    exitProgress = Math.min(elapsed / $settings.exitHoldDuration, 1);
    
    if (exitProgress >= 1) {
      console.log('Exiting...');
      invoke('power_action', { action: 'shutdown' }); // Actually closing the window might be better via tauri API, but we can trigger close. Or we can just use process exit.
      // Alternatively, we just use standard window close since we want to exit Gaming Hub
      import('@tauri-apps/api/window').then(({ getCurrentWindow }) => {
        getCurrentWindow().close();
      });
    } else {
      animationFrame = requestAnimationFrame(updateProgress);
    }
  }
  
  onMount(() => {
    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      cancelAnimationFrame(animationFrame);
    };
  });
</script>

{#if isHoldingEsc && exitProgress > 0}
  <div class="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-black/80 backdrop-blur-sm transition-opacity duration-300">
    <div class="text-2xl font-light mb-8">Hold ESC to exit</div>
    
    <div class="w-96 h-2 bg-gaming-surface rounded-full overflow-hidden mb-4">
      <div 
        class="h-full bg-white transition-none" 
        style="width: {exitProgress * 100}%"
      ></div>
    </div>
    
    <div class="text-gaming-muted font-mono">
      {((exitProgress * $settings.exitHoldDuration) / 1000).toFixed(1)} / {($settings.exitHoldDuration / 1000).toFixed(1)} seconds
    </div>
  </div>
{/if}
