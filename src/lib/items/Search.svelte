<script lang="ts">
   import type { search } from "../../items"
   import { queryResult } from "../../store"
   const srch: search = {
      name: "Search...",
      query: "",
   };
   const submit = (e: Event) => {
      //store input value in search object
      srch.query = (<HTMLInputElement>e.target).value;
      queryResult.update(() => srch.query);
   };
</script>

<div class="search-div-outer">
   <div class="search-div-inner">
      <i class="fa-solid fa-magnifying-glass search-icon"></i>
      <input class="search-input" on:change={submit} type="text" placeholder="{srch.name}">
      {srch.query}
   </div>
</div>

<style>
   .search-icon {
      padding-right: .15vw;
      color: #79797998;
      font-size: 2.25vh;
      font-weight: bolder;
   }
   .search-input {
      background-color: transparent;
      border-color: transparent;
      outline: none;
      color: #46464698;
      font-size: 2.25vh;
      font-weight: bolder;
      width: 10vw;
      caret-color: black;
   }
   .search-div-outer {
      width: 100%;
      height: 6vh;
      display: flex;
      justify-content: center;
   }
   .search-div-inner {
      margin-top: 8vh;
      align-items: center;
      justify-content: center;
      display: flex;
      height: 6vh;
      border-radius: 1vw;
      border: 2px solid rgba(0, 106, 255, 0.545);
      background-color: rgba(122, 224, 255, 0.469);
      font-variant-numeric: tabular-nums;
      padding: .15em 1em;
   }
</style>