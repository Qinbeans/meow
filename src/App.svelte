<script lang="ts">
  import logo from './assets/meow.png'
  import Sidepanel from './lib/Sidepanel.svelte'
</script>

<Sidepanel title={"Meow"}/>
<main>
  <img src={logo} alt="Logo" class="main-logo"/>
</main>

<style>
  .main-logo {
    width: 15vw;
    height: 15vw;
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
