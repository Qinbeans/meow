<script lang="ts">
  import logo from './assets/svelte.png'
  import Sidepanel from './lib/Sidepanel.svelte'
</script>

<Sidepanel title={"Meow"}/>
<main>
  <i class="fa-solid fa-cat catify"></i>
</main>

<style>
  .catify {
    font-size: 100px;
    color: rgba(0, 149, 255, 0.786);
  }

  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    padding: 1em;
    margin: 0 auto;
    display: flex;
    text-align: center;
    flex-direction: column;
    align-items: center;
  }
/* 
  @media (min-width: 480px) {
    h1 {
      max-width: none;
    }

    p {
      max-width: none;
    }
  } */
</style>
