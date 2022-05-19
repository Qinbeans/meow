{#if toggleStatus}
   <div class="side-panel" transition:fly={{x: -500, opacity: 1}}>
      {#if !is_home}
         <button class="side-toggle decorator rs-5vw lu-corner" on:click={()=>{is_home=true}}>
            {title}
         </button>
      {:else}
         <h1 class="side-title decorator lu-corner">
            {title}
         </h1>
      {/if}
      <Search/>
   </div>
{/if}
<button class="side-toggle decorator lu-corner" on:click={toggleSidebar}>
   {#if toggleStatus}
      <i class="fa-solid fa-angles-left"></i>
      {:else}
      <i class="fa-solid fa-angles-right"></i>
   {/if}
</button>

<script lang="ts">
   import Search from "./items/Search.svelte";
   import { fly } from 'svelte/transition';
   //toggle the visibility of the sidebar
   export let title = "";
   export let is_home = true;
   let toggleStatus = false
   const toggleSidebar = () => {
     toggleStatus = !toggleStatus
   }
</script>