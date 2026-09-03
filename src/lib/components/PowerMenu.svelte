<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  let confirming = $state<string | null>(null);
  
  const actions = [
    { id: 'sleep', label: 'Sleep', icon: '☾' },
    { id: 'restart', label: 'Restart', icon: '↻' },
    { id: 'shutdown', label: 'Shut Down', icon: '⏻' }
  ];
  
  async function handleAction(actionId: string) {
    if (confirming !== actionId) {
      confirming = actionId;
      return;
    }
    
    console.log(`Executing power action: ${actionId}`);
    try {
      await invoke('power_action', { action: actionId });
    } catch (e) {
      console.error('Power action failed:', e);
    }
    confirming = null;
  }
</script>

<div class="glass-panel rounded-2xl p-6 h-full flex flex-col">
  <h3 class="text-lg font-bold mb-6 text-gaming-muted tracking-wide">POWER</h3>
  
  <div class="flex-1 flex flex-col gap-3">
    {#each actions as action}
      <button 
        class="flex items-center justify-between p-4 rounded-xl transition-all duration-200 hover:bg-white/10 hover:scale-[1.02] active:scale-95 group text-left"
        onclick={() => handleAction(action.id)}
      >
        <div class="flex items-center gap-4 text-xl">
          <span class="w-8 text-center text-gaming-muted group-hover:text-white transition-colors">{action.icon}</span>
          <span>{action.label}</span>
        </div>
        
        {#if confirming === action.id}
          <span class="text-red-400 font-bold text-sm bg-red-900/30 px-3 py-1 rounded-full animate-pulse">
            Confirm?
          </span>
        {/if}
      </button>
    {/each}
  </div>
</div>
