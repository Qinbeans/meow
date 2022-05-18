<script lang="ts">
   import Search from "./items/Search.svelte";

   //toggle the visibility of the sidebar
   export let title = "";
   let toggleStatus = false
   const toggleSidebar = () => {
     toggleStatus = !toggleStatus
   }
</script>

{#if toggleStatus}
   <div class="side-panel">
      <button class="side-toggle" on:click={toggleSidebar}>
         {#if toggleStatus}
            <i class="fa-solid fa-angles-left"></i>
            {:else}
            <i class="fa-solid fa-angles-right"></i>
         {/if}
      </button>
      <h1 class="side-title">
         {title}
      </h1>
      <Search />
   </div>
{:else}
   <button class="side-toggle" on:click={toggleSidebar}>
      {#if toggleStatus}
         <i class="fa-solid fa-angles-left"></i>
         {:else}
         <i class="fa-solid fa-angles-right"></i>
      {/if}
   </button>
{/if}

<style>
   button.side-toggle {
      position: fixed;
      display: flex;
      margin: .5vw 0 .5vw .5vw;
      top: 0;
      left: 0;
      font-family: inherit;
      font-size: 3vh;
      padding: 1vh 1vw;
      color: #006aff;
      background-color: rgba(122, 224, 255, 0.469);
      border-radius: 1vw;
      border: 2px solid rgba(0, 106, 255, 0.545);
      outline: none;
      font-variant-numeric: tabular-nums;
      cursor: pointer;
   }
 
   button:focus.side-toggle {
     border: 2px solid #0091ff;
   }
 
   button:active.side-toggle {
     background-color: rgba(153, 0, 255, 0.459);
   }

   .side-title{
      margin: .5vw 0 .5vw 5vw;
      position: fixed;
      display: flex;
      font-family: inherit;
      font-size: 3vh;
      height: 3.5vh;
      padding: 1vh 1.5vw;
      color: #006aff;
      background-color: rgba(122, 224, 255, 0.469);
      border-radius: 1vw;
      border: 2px solid rgba(0, 106, 255, 0.545);
      outline: none;
      font-variant-numeric: tabular-nums;
   }

   .side-panel {
      position: fixed;
      top: 0;
      left: 0;
      width: 18vw;
      height: 100%;
      background-color: rgb(113, 113, 113);
      display: flex;
      border-radius: 0 1em 1em 0;
   }
 </style> 