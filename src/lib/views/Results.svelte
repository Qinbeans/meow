<script lang="ts">
    import { queryResult } from '../../store'
    import type { Response } from '../../types'
    let q_res: Response[]
    queryResult.subscribe((res: Response[]) => {
        q_res = res
    })
</script>
<div class="grid-container w-70vw h-70vw ts-10vw">
    {#each q_res as res }
        <div class="outter-res">
            <img src="{res.thumbnail}" alt="{res.id}">
            <span>{res.name}</span>
        </div>
    {/each}
</div>