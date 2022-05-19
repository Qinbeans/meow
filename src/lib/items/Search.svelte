<div class="search-div-outer">
   <div class="search-div-inner">
      <i class="fa-solid fa-magnifying-glass search-icon"></i>
      <input class="search-input" on:change={submit} type="text" placeholder="{srch.name}">
   </div>
</div>

<script lang="ts">
   import type { search } from "../../items"
   import { queryResult, err } from "../../store"
   import type { Response } from "../../types"
   import { tauri } from "@tauri-apps/api"
   const srch: search = {
      name: "Search...",
      query: "",
   };
   const submit = (e: Event) => {
      //store input value in search object
      srch.query = (<HTMLInputElement>e.target).value
      if(srch.query == ""){
         return
      }
      const query = srch.query;
      let result: Response[];
      tauri
         .invoke("youtube", {url: query})
         .then((response: string) => {
            result = JSON.parse(response)
            result.forEach((item) => {
               result.push(item);
            });
            queryResult.update(() => result)
         })
         .catch((error: string) => {
            err.update(() => error)
         });
   };
</script>