<Sidepanel title={"Meow"}/>
{#if is_home}
  <Logo/>
{:else}
  <Results/>
{/if}

<script lang="ts">
  import Logo from "./lib/views/Logo.svelte"
  import Sidepanel from './lib/Sidepanel.svelte'
  import Results from "./lib/views/Results.svelte";
  import { queryResult } from './store'
  import type { Response } from './types'
  let is_home: boolean = true
  queryResult.subscribe((res: Response[]) => {
    if(res==undefined){
      is_home = true
      return
    }
    console.log(res)
    is_home = (res.length < 1)
  })
</script>
